use ddose_common::{CanFrame, CanInterface, CanSocket};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

/// Allows reading and writing frames on a CAN bus.
///
/// [CanBus] provides access to an socketcan interface. using [CanBus::read()]
/// and [CanBus::write()], frames can be received and transmitted.
pub struct CanBus {
    socket: CanSocket,
}

impl CanBus {
    /// Opens a CAN bus
    ///
    /// Creates a new socketcan socket and binds it to the specified interface.
    pub fn open(can_if: &CanInterface) -> Result<Self, std::io::Error> {
        let socket = CanSocket::create()?;
        socket.bind(&can_if)?;
        socket.set_nonblocking()?;

        Ok(Self { socket })
    }

    pub async fn read(&mut self) -> Result<CanFrame, std::io::Error> {
        const FRAME_SIZE: usize = std::mem::size_of::<libc::can_frame>();
        let mut buffer = [0; FRAME_SIZE];

        let bytes_read = self.socket.read(&mut buffer).await?;
        if bytes_read != FRAME_SIZE {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Received incomplete CAN frame",
            ));
        }

        let frame = unsafe { std::mem::transmute::<[u8; FRAME_SIZE], libc::can_frame>(buffer) };
        Ok(CanFrame::from_inner(frame))
    }

    pub async fn write(&mut self, can_frame: &CanFrame) -> Result<(), std::io::Error> {
        const FRAME_SIZE: usize = std::mem::size_of::<libc::can_frame>();
        // TODO: remove unnecessary copy
        let bytes = unsafe {
            std::mem::transmute_copy::<libc::can_frame, [u8; FRAME_SIZE]>(can_frame.inner())
        };

        if self.socket.write(&bytes).await? != FRAME_SIZE {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Received incomplete CAN frame",
            ));
        }

        Ok(())
    }
}
