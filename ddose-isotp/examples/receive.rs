use ddose_common::CanInterface;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let can_if = CanInterface::try_from("vcan0")?;
    let rx_id = embedded_hal::can::Id::Standard(embedded_hal::can::StandardId::new(0x100).unwrap());
    let tx_id = embedded_hal::can::Id::Standard(embedded_hal::can::StandardId::new(0x101).unwrap());

    let mut isotp_connection = ddose_isotp::IsotpConnection::open(&can_if, tx_id, rx_id)?;

    let mut buffer = [0; 4096];
    loop {
        let bytes_read = isotp_connection.read(&mut buffer).await?;
        println!("[{}] {:02X?}", bytes_read, &buffer[..bytes_read]);
    }
}
