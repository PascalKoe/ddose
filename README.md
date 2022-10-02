# DDose
**This project is still under developement. Don't use it in any production systems and under no circumstances in safety critical applications!**

DDose provides you the tools access your CAN interfaces on Linux. Under the
hood, the socketcan API of the Linux kernel is used. This allows you to use
different CAN interfaces litke the PCAN interfaces form Peak Symstes, the PiCan
shields for a Raspberry Pi or any other socketcan compatible interface.
For more information about socketcan, visit the [Linux Kernel documentation](
https://www.kernel.org/doc/html/latest/networking/can.html).

 * `CanBus` allows you to receive and send raw CAN frames.
 * `IsotpConnection` allows you to send and receive large payloads.
 * `UdsClient`allows you to access the diagnostics interface on automotive ECUs

DDose currently is build for the use with the async Tokio Runtime. There is no
plan to support sync environments but if you need it, feel free to open a pull
request!

## Usage
In order to access the CAN bus, you first need to define which interface you want to access. You can either access the interface by its name or by its index.

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

## Credits
Credits go out to all of the other awesome individuals which implemented crates to access CAN interfaces using socketcan. Even though I successfully used some of the existing crates, I found myself wanting a single compatible stack for CAN, ISOTP and UDS. 

So many thanks and also credits go out to all of the contributors of the following projects:

 * [rnd-ash/ecu_diagnostics](https://github.com/rnd-ash/ecu_diagnostics)
 * [marcelbuesing/socketcan-isotp](https://github.com/marcelbuesing/socketcan-isotp)
 * [oefd/tokio-socketcan](https://github.com/oefd/tokio-socketcan)
 * [socketcan-rs/socketcan-rs](https://github.com/socketcan-rs/socketcan-rs)

## License
Licensed under either of

 * Apache License Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
