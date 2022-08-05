/*!
This crate allows you to access your CAN bus. Under the hood, the
socketcan API of the Linux kernel is used. This allows you to use
different CAN interfaces like the PCAN interfaces from Peak Systems,
the PiCan shields for the Raspberry Pi or any other compatible
interface.

For more information about socketcan, visit the [Linux Kernel
documentation](https://www.kernel.org/doc/html/latest/networking/can.html).

# Example
```
use ddose_can::{CanInterface, CanBus};

let can_if = CanInterface::try_from("vcan0").unwrap();
let mut can_bus = CanBus::open(&can_if).unwrap();

// Echo incoming frames
loop {
    let frame = can_bus.read().await.unwrap();
    can_bus.write(&frame).await.unwrap();
}
```
*/

mod bus;
mod frame;
mod socket;

pub use bus::*;
pub use frame::*;
pub use socket::*;
