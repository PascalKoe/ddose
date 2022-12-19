use std::os::unix::prelude::AsRawFd;

use embedded_hal::can::Id as CanId;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::socket::{CanInterface, CanSocket};

pub struct IsotpConnection {
    socket: CanSocket,
}

impl IsotpConnection {
    pub fn open(
        can_if: &CanInterface,
        tx_id: impl Into<CanId>,
        rx_id: impl Into<CanId>,
    ) -> Result<Self, std::io::Error> {
        let socket = CanSocket::create(libc::SOCK_DGRAM, libc::CAN_ISOTP)?;
        socket.set_nonblocking()?;

        let fd = socket.as_raw_fd();
        let mut isotp_options = IsoTpOptions::new();
        isotp_options.set_flag(IsotpOptionsFlag::WaitTxDone);

        let isotp_options_ptr: *const libc::c_void =
            &isotp_options as *const _ as *const libc::c_void;

        let _ = unsafe {
            libc::setsockopt(
                fd,
                libc::SOL_CAN_BASE + libc::CAN_ISOTP,
                1, // Constant...
                isotp_options_ptr,
                ISOTP_OPTIONS_SIZE.try_into().unwrap(),
            )
        };

        // The socket must be bound to the specific ISOTP TX and RX IDs
        let can_addr = Self::can_address(rx_id, tx_id);
        socket.bind_address(can_if, can_addr)?;

        Ok(Self { socket })
    }

    pub async fn read(&mut self, buffer: &mut [u8]) -> Result<usize, std::io::Error> {
        self.socket.read(buffer).await
    }

    pub async fn write(&mut self, buffer: &[u8]) -> Result<usize, std::io::Error> {
        self.socket.write(buffer).await
    }

    fn can_address(
        tx_id: impl Into<CanId>,
        rx_id: impl Into<CanId>,
    ) -> libc::__c_anonymous_sockaddr_can_can_addr {
        let mut address: libc::__c_anonymous_sockaddr_can_can_addr = unsafe { std::mem::zeroed() };
        address.tp.rx_id = match rx_id.into() {
            CanId::Standard(id) => id.as_raw() as u32,
            CanId::Extended(id) => id.as_raw(),
        };
        address.tp.tx_id = match tx_id.into() {
            CanId::Standard(id) => id.as_raw() as u32,
            CanId::Extended(id) => id.as_raw(),
        };

        address
    }
}

const ISOTP_OPTIONS_SIZE: usize = std::mem::size_of::<IsoTpOptions>();

#[repr(u16)]
enum IsotpOptionsFlag {
    /// Listen only (do not send FC)
    ListenMode = 0x0001,
    /// Enable extended addressing
    ExtendedAddr = 0x0002,
    /// Enable CAN frame padding tx path
    TxPadding = 0x0004,
    /// Enable CAN frame padding rx path
    RxPadding = 0x0008,
    /// Check received CAN frame padding
    CheckPadLen = 0x0010,
    /// Check received CAN frame padding
    ChkPadData = 0x0020,
    /// Half duplex error state handling
    HalfDuplex = 0x0040,
    /// Ignore stmin from received FC
    ForceTxStMin = 0x0080,
    /// Ignore CFs depending on rx stmin
    ForceRxStMin = 0x0100,
    /// Different rx extended addressing
    RxExtAddr = 0x0200,
    /// Wait for tx completion
    WaitTxDone = 0x0400,
    /// 1-to-N functional addressing
    SfBroadcast = 0x0800,
    /// 1-to-N transmission w/o FC
    CfBroadcast = 0x1000,
}

#[repr(C)]
struct IsoTpOptions {
    /// set flags for isotp behaviour.
    flags: u32,
    /// frame transmission time (N_As/N_Ar)
    /// time in nano secs
    frame_txtime: u32,
    /// set address for extended addressing
    ext_address: u8,
    /// set content of padding byte (tx)
    txpad_content: u8,
    /// set content of padding byte (rx)
    rxpad_content: u8,
    /// set address for extended addressing
    rx_ext_address: u8,
}

impl IsoTpOptions {
    pub fn new() -> Self {
        Self {
            flags: 0x00,
            frame_txtime: 0x00,
            ext_address: 0x00,
            txpad_content: 0xCC,
            rxpad_content: 0xCC,
            rx_ext_address: 0x00,
        }
    }

    pub fn set_flag(&mut self, flag: IsotpOptionsFlag) {
        self.flags = self.flags | flag as u32;
    }

    pub fn clear_flag(&mut self, flag: IsotpOptionsFlag) {
        self.flags = self.flags & !(flag as u32);
    }
}
