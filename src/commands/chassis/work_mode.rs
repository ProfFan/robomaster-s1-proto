//! The RM-S1 Work Mode Set command

use crate::wire::RMWireFrameView;

/// The RM-S1 Work Mode Set command
pub struct ChassisWorkModeSetView<T: AsRef<[u8]>> {
    pub packet: RMWireFrameView<T>,
}

impl<T: AsRef<[u8]>> ChassisWorkModeSetView<T> {
    pub fn new(packet: RMWireFrameView<T>) -> Self {
        Self { packet }
    }

    pub fn work_mode(&self) -> u8 {
        self.packet.payload()[0]
    }
}

impl<T: AsRef<[u8]> + AsMut<[u8]>> ChassisWorkModeSetView<T> {
    pub fn set_work_mode(&mut self, work_mode: u8) {
        self.packet.payload_mut()[0] = work_mode;
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
    fn test_workmodesetview() {
        let buf = [
            0x55, 0x1B, 0x04, 0x75, 0x09, 0xC3, 0xE0, 0x00, 0x00, 0x3F, 0x19, 0x01, 0x04, 0x20,
        ];

        let packet = RMWireFrameView::new(&buf);
        let work_mode_set = ChassisWorkModeSetView::new(packet);

        assert_eq!(work_mode_set.packet.cmd_set(), CommandSetType::RM as u8);
        assert_eq!(
            work_mode_set.packet.cmd_id(),
            RMCommandType::WORK_MODE_SET as u8
        );
        assert_eq!(work_mode_set.work_mode(), 0x01); // SDK Mode
    }
}
