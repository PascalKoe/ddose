/*!
DDose provides you the tools access your CAN interfaces on Linux. Under the
hood, the socketcan API of the Linux kernel is used. This allows you to use
different CAN interfaces litke the PCAN interfaces form Peak Symstes, the PiCan
shields for a Raspberry Pi or any other socketcan compatible interface.
For more information about socketcan, visit the [Linux Kernel documentation](
https://www.kernel.org/doc/html/latest/networking/can.html).

 * [CanBus] allows you to receive and send raw CAN frames.
 * [IsotpConnection] allows you to send and receive large payloads.
 * [UdsClient](crate::uds::UdsClient) allows you to access the diagnostics
   interface on automotive ECUs

DDose currently is build for the use with the async Tokio Runtime. There is no
plan to support sync environments but if you need it, feel free to open a pull
request!

# Examples
In order to access the CAN bus, you first need to define which interface you
want to access. You can either access the interface by its name or by its index.

```rust
use ddose::CanInterface;

// Interface by name (e.g., vcan0, can0, socan0, ...)
let can_if = CanInterface::try_from("can0").unwrap();
// or by interface index
let can_if = CanInterface::try_from(4).unwrap();
```

The send and receive raw CAN frames, you can use the `CanBus`.
```rust
use ddose::CanBus;
let mut can_bus = CanBus::open(&can_if).unwrap();

// Read from the CAN bus
let frame = can_bus.read().await.unwrap();

// Write to the CAN bus
let can_bus.write(&frame).await.unwrap();
```

To send large payloads using ISOTP, you can use the `IsotpConnection`.
```rust
use ddose::IsotpConnection;
let rx_id = embedded_hal::can::StandardId::new(0x100).unwrap();
let tx_id = embedded_hal::can::StandardId::new(0x101).unwrap();
let mut isotp_conn = IsotpConnection::open(&can_if, tx_id, rx_id).unwrap();

// Receive data from another ISOTP device
let mut buffer = [0; 4096];
let bytes_read = isotp_conn.read(&mut buffer).await.unwrap();
let payload = &buffer[..bytes_read];

// Echo back the received data
let _bytes_written = isotp_conn.write(&buffer).await.unwrap();
```
*/

mod can;
mod isotp;
mod socket;

pub mod uds;

pub use can::*;
pub use isotp::*;
pub use socket::*;
