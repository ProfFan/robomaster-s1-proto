# `robomaster-s1-proto`

`robomaster-s1-proto` is a `no_std` Rust library for the DJI RoboMaster S1 robot.

We provide safe interfaces for the S1's CAN bus protocol.

Features include:
- Chassis/Blaster/Gimbal control
- DUSS and DUSS VBUS (Virtual Bus) Pub/Sub interfaces
- Protocol parser for the S1's CAN bus messages

# Tools

We include a decoder tool for parsing a CAN dump with the Linux `can-utils` `candump` tool. To use it, run

```sh
cargo run --example rm-can-decode data.log
```

# LICENSE

MIT OR Apache-2.0
