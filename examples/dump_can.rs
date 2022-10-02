use ddose::{CanInterface, CanBus};
use embedded_hal::can::Frame;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let can_if = CanInterface::try_from("vcan0")?;
    let mut can_bus = CanBus::open(&can_if)?;

    loop {
        let frame = can_bus.read().await?;
        let id = match frame.id() {
            embedded_hal::can::Id::Standard(id) => id.as_raw() as u32,
            embedded_hal::can::Id::Extended(id) => id.as_raw(),
        };
        println!("[{:03X}] {:02X?}", id, &frame.data()[..frame.dlc()]);
    }
}