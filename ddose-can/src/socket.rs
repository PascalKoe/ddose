use std::{
    ffi::CString,
    os::unix::{io::RawFd, prelude::AsRawFd},
};

use tokio::io::{unix::AsyncFd, AsyncRead, AsyncWrite};

/// Represents a specific CAN interface on the system.
///
/// The CAN interace can either be addressed by an name (e.g., `vcan0`)
/// or by its index.
/// During the creation, it is checked if the interface actually exists.
/// If the interface doesn't exist, an error is returned.
///
/// # Example:
/// ```
/// let interface = match CanInterface::try_from("vcan0") {
///     Ok(if) => if,
///     Err(e) => {
///         println!("Couldn't find the CAN interface: {}", e);
///         exit();
///     }
/// };
/// ```
pub struct CanInterface(libc::c_uint);

impl CanInterface {
    /// Returns the interfaces system index
    pub fn if_index(&self) -> libc::c_uint {
        self.0
    }
}

impl TryFrom<&str> for CanInterface {
    type Error = std::io::Error;

    fn try_from(if_name: &str) -> Result<Self, Self::Error> {
        let if_name = CString::new(if_name)?;
        let if_index = unsafe { libc::if_nametoindex(if_name.as_ptr()) };

        if if_index == 0 {
            return Err(std::io::Error::last_os_error());
        }

        Ok(Self(if_index))
    }
}

impl TryFrom<libc::c_uint> for CanInterface {
    type Error = std::io::Error;

    fn try_from(if_index: libc::c_uint) -> Result<Self, Self::Error> {
        let mut if_name: [libc::c_char; libc::IF_NAMESIZE] = [0; libc::IF_NAMESIZE];
        let ptr = if_name.as_mut_ptr();
        let ret = unsafe { libc::if_indextoname(if_index, ptr) };

        // We know it is an existing interface but we don't know
        // if it's actaully a socketcan interface
        if ret != ptr as _ {
            return Err(std::io::Error::last_os_error());
        };

        Ok(Self(if_index))
    }
}

pub(crate) struct CanSocket(AsyncFd<RawFd>);

impl CanSocket {
    /// Creates a new Linux socket
    ///
    /// This function creates a new CAN socket. The socket is not bound to any
    /// CAN interface and therefore no I/O operations are available.
    /// To bind the socket, call [CanSocket::bind()].
    pub fn create() -> Result<Self, std::io::Error> {
        let socket = unsafe { libc::socket(libc::PF_CAN, libc::SOCK_RAW, libc::CAN_RAW) };
        if socket.is_negative() {
            return Err(std::io::Error::last_os_error());
        }

        Ok(Self(AsyncFd::new(socket)?))
    }

    /// Binds the CAN socket to an CAN interface
    ///
    /// After the CAN socket is open, the interface which the CAN socket shall
    /// listen on or write to must be definied.
    pub fn bind(&self, can_if: &CanInterface) -> Result<(), std::io::Error> {
        const ADDRESS_SIZE: usize = std::mem::size_of::<libc::sockaddr_can>();

        let mut address: libc::sockaddr_can = unsafe { std::mem::zeroed() };
        address.can_family = libc::AF_CAN as _;
        address.can_ifindex = can_if.if_index() as _;

        let ptr = &address as *const libc::sockaddr_can;
        let ret = unsafe { libc::bind(self.as_raw_fd(), ptr as _, ADDRESS_SIZE as _) };
        if ret == -1 {
            // Store error because closing could produce another error
            let error = std::io::Error::last_os_error();
            unsafe {
                libc::close(self.as_raw_fd());
            }
            return Err(error);
        }

        Ok(())
    }

    /// Sets the O_NOBLOCK flag on the socket
    ///
    /// This function sets the O_NOBLOCK flag on the underlying socket. This is
    /// required for async to work.
    pub fn set_nonblocking(&self) -> Result<(), std::io::Error> {
        // Get current flags so we can only change the O_NOBLOCK flag
        let mut flags = unsafe { libc::fcntl(self.as_raw_fd(), libc::F_GETFL) };
        if flags == -1 {
            return Err(std::io::Error::last_os_error());
        }

        // Write back the flags with O_NOBLOCK set
        flags |= libc::O_NONBLOCK;
        let ret = unsafe { libc::fcntl(self.as_raw_fd(), libc::F_SETFL, flags) };
        if ret == -1 {
            return Err(std::io::Error::last_os_error());
        }

        Ok(())
    }
}

impl std::os::unix::io::AsRawFd for CanSocket {
    fn as_raw_fd(&self) -> RawFd {
        *self.0.get_ref()
    }
}

impl AsyncRead for CanSocket {
    fn poll_read(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        loop {
            let mut ready = match self.0.poll_read_ready(cx) {
                std::task::Poll::Ready(t) => t,
                std::task::Poll::Pending => return std::task::Poll::Pending,
            }?;

            let ret = unsafe {
                libc::read(
                    *self.0.get_ref(),
                    buf.unfilled_mut() as *mut _ as _,
                    buf.remaining(),
                )
            };

            if ret.is_negative() {
                let error = std::io::Error::last_os_error();
                match error.kind() {
                    std::io::ErrorKind::WouldBlock => ready.clear_ready(),
                    _ => return std::task::Poll::Ready(Err(error)),
                }
            } else {
                let n_bytes = ret as usize;
                unsafe { buf.assume_init(n_bytes) };
                buf.advance(n_bytes);
                return std::task::Poll::Ready(Ok(()));
            }
        }
    }
}

impl AsyncWrite for CanSocket {
    fn poll_write(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> std::task::Poll<Result<usize, std::io::Error>> {
        loop {
            let mut ready = match self.0.poll_write_ready(cx) {
                std::task::Poll::Ready(t) => t,
                std::task::Poll::Pending => return std::task::Poll::Pending,
            }?;

            let ret = unsafe { libc::write(*self.0.get_ref(), buf.as_ptr() as _, buf.len()) };
            if ret.is_negative() {
                let error = std::io::Error::last_os_error();
                match error.kind() {
                    std::io::ErrorKind::WouldBlock => ready.clear_ready(),
                    _ => return std::task::Poll::Ready(Err(error)),
                }
            } else {
                let n_bytes = ret as usize;
                return std::task::Poll::Ready(Ok(n_bytes));
            }
        }
    }

    fn poll_flush(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), std::io::Error>> {
        std::task::Poll::Ready(Ok(()))
    }

    fn poll_shutdown(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), std::io::Error>> {
        std::task::Poll::Ready(Ok(()))
    }
}
