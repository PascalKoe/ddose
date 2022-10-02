use ddose::{CanInterface, IsotpConnection};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let can_if = CanInterface::try_from("vcan0")?;
    let rx_id = embedded_hal::can::StandardId::new(0x100).unwrap();
    let tx_id = embedded_hal::can::StandardId::new(0x101).unwrap();

    let mut isotp_connection = IsotpConnection::open(&can_if, tx_id, rx_id)?;
    let bytes_written = isotp_connection.write(&[0xFE; 32]).await?;

    println!("{}", bytes_written);

    Ok(())
}
