use crate::{
    duss::{cmd_set_rm::RMCommandType, cmd_set_types::CommandSetType},
    wire::RMWireFrameView,
};

/// Chassis peed vector control packet
#[derive(Debug)]
pub struct SpeedSetView<T: AsRef<[u8]>> {
    pub packet: RMWireFrameView<T>,
}

impl<T: AsRef<[u8]>> SpeedSetView<T> {
    pub fn new(packet: RMWireFrameView<T>) -> Self {
        Self { packet }
    }

    pub fn x(&self) -> f32 {
        f32::from_le_bytes([
            self.packet.payload()[0],
            self.packet.payload()[1],
            self.packet.payload()[2],
            self.packet.payload()[3],
        ])
    }

    pub fn y(&self) -> f32 {
        f32::from_le_bytes([
            self.packet.payload()[4],
            self.packet.payload()[5],
            self.packet.payload()[6],
            self.packet.payload()[7],
        ])
    }

    pub fn omega(&self) -> f32 {
        f32::from_le_bytes([
            self.packet.payload()[8],
            self.packet.payload()[9],
            self.packet.payload()[10],
            self.packet.payload()[11],
        ])
    }

    pub fn is_valid(&self) -> bool {
        self.packet.is_valid()
            && self.packet.payload().len() == 12
            && self.packet.cmd_set() == CommandSetType::RM as u8
            && self.packet.cmd_id() == RMCommandType::SPEED_SET as u8
    }
}

impl<T: AsRef<[u8]> + AsMut<[u8]>> SpeedSetView<T> {
    pub fn set_x(&mut self, x: f32) {
        self.packet.payload_mut()[0..4].copy_from_slice(&x.to_le_bytes());
    }

    pub fn set_y(&mut self, y: f32) {
        self.packet.payload_mut()[4..8].copy_from_slice(&y.to_le_bytes());
    }

    pub fn set_omega(&mut self, omega: f32) {
        self.packet.payload_mut()[8..12].copy_from_slice(&omega.to_le_bytes());
    }
}

#[cfg(test)]
mod tests {
    extern crate std;
    use crate::duss::{cmd_set_rm::RMCommandType, cmd_set_types::CommandSetType};

    use super::*;

    #[test]
    fn test_speed_vector_control_view() {
        let mut buf = [
            0x55, 0x19, 0x04, 0xe4, 0x09, 0xC3, 0xE0, 0x00, 0x00, 0x3F, 0x21, //
            0x00, 0x00, 0x80, 0x3f, //
            0x00, 0x00, 0x80, 0x3f, //
            0x00, 0x00, 0x80, 0x3f, //
            0x7d, 0x16,
        ];

        let packet = RMWireFrameView::new(&mut buf);
        let mut speed_vector_control = SpeedSetView::new(packet);

        assert_eq!(
            speed_vector_control.packet.cmd_set(),
            CommandSetType::RM as u8
        );
        assert_eq!(
            speed_vector_control.packet.cmd_id(),
            RMCommandType::SPEED_SET as u8
        );
        assert_eq!(speed_vector_control.packet.payload().len(), 3 * 4);

        assert_eq!(speed_vector_control.x(), 1.0);
        assert_eq!(speed_vector_control.y(), 1.0);
        assert_eq!(speed_vector_control.omega(), 1.0);

        assert_eq!(speed_vector_control.is_valid(), true);

        speed_vector_control.set_x(2.0);
        assert_eq!(speed_vector_control.x(), 2.0);
    }
}
