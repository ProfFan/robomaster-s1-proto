//! The RM-S1 RMC message format
//!
//! NOTE: This is a work in progress and may not be accurate

use crate::wire::RMWireFrameView;

/// The RM-S1 RMC message format
pub struct RMCPacketView<T: AsRef<[u8]>> {
    pub packet: RMWireFrameView<T>,
}

impl<T: AsRef<[u8]>> RMCPacketView<T> {
    pub fn new(packet: RMWireFrameView<T>) -> Self {
        Self { packet }
    }
}

#[cfg(test)]
mod tests {
    extern crate std;
    use super::*;
    use crate::{
        duss::{cmd_set_rm::RMCommandType, cmd_set_types::CommandSetType},
        wire::RMWireFrameView,
    };

    #[test]
    fn test_rmcpacketview() {
        let buf = [
            0x55, 0x1B, 0x04, 0x75, 0x09, 0xC3, 0xE0, 0x00, 0x00, 0x3F, 0x60, 0x00, 0x04, 0x20,
            0x00, 0x01, 0x00, 0x40, 0x00, 0x02, 0x10, 0x04, 0x03, 0x00, 0x04, 0xFA, 0xF0,
        ];

        let packet = RMWireFrameView::new(&buf);
        let rmc_packet = RMCPacketView::new(packet);

        assert_eq!(rmc_packet.packet.is_valid(), true);

        assert_eq!(rmc_packet.packet.cmd_set(), CommandSetType::RM as u8);
        assert_eq!(rmc_packet.packet.cmd_id(), RMCommandType::FC_RMC as u8);

        std::println!("{:0x?}", rmc_packet.packet);
    }
}
