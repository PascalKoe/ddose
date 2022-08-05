use ddose_common::{CanInterface, CanSocket};

use embedded_hal::can::Id as CanId;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

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
