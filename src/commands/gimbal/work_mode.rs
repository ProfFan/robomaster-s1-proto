//! The RM-S1 Gimbal Work Mode Set command

use crate::wire::RMWireFrameView;
use num_enum::TryFromPrimitive;

/// The RM-S1 Gimbal Work Mode
#[derive(Debug, TryFromPrimitive, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum GimbalMode {
    Free = 0x00,
    FPV = 0x01,
    Follow = 0x02,
    Unknown = 0xFF,
}

/// The RM-S1 Gimbal Work Mode Set command
pub struct GimbalWorkModeSetView<T: AsRef<[u8]>> {
    pub packet: RMWireFrameView<T>,
}

impl<T: AsRef<[u8]>> GimbalWorkModeSetView<T> {
    pub fn new(packet: RMWireFrameView<T>) -> Self {
        Self { packet }
    }

    pub fn mode(&self) -> GimbalMode {
        self.packet.payload()[0]
            .try_into()
            .unwrap_or(GimbalMode::Unknown)
    }

    pub fn cmd(&self) -> u8 {
        self.packet.payload()[1]
    }

    pub fn is_valid(&self) -> bool {
        self.packet.is_valid() && self.cmd() == 0x00
    }
}

impl<T: AsRef<[u8]> + AsMut<[u8]>> GimbalWorkModeSetView<T> {
    pub fn set_mode(&mut self, work_mode: u8) {
        self.packet.payload_mut()[0] = work_mode;
    }

    pub fn set_cmd(&mut self, cmd: u8) {
        self.packet.payload_mut()[1] = cmd;
    }
}

#[cfg(test)]
mod tests {
    extern crate std;
    use super::*;
    use crate::{
        duss::{cmd_set_gimbal::GimbalCommandType, cmd_set_types::CommandSetType},
        wire::RMWireFrameView,
    };

    #[test]
    fn test_workmodesetview() {
        let buf = [
            0x55, 0x0F, 0x04, 0xa2, 0x09, 0xC3, 0xE0, 0x00, 0x00, 0x04, 0x4C, 0x00, 0x00, 0x6c,
            0xe1,
        ];

        let packet = RMWireFrameView::new(&buf);
        let work_mode_set = GimbalWorkModeSetView::new(packet);

        assert_eq!(work_mode_set.packet.cmd_set(), CommandSetType::GIMBAL as u8);
        assert_eq!(
            work_mode_set.packet.cmd_id(),
            GimbalCommandType::GIMBAL_SET_MODE as u8
        );
        assert_eq!(work_mode_set.mode(), GimbalMode::Free); // Free Mode
        assert_eq!(work_mode_set.cmd(), 0x00); // Always 0x00

        // std::println!(
        //     "CRC8: {:0x}, CRC16: {:0x}",
        //     work_mode_set.packet.crc8_computed(),
        //     work_mode_set.packet.crc16_computed()
        // );

        assert_eq!(work_mode_set.is_valid(), true);
    }
}
