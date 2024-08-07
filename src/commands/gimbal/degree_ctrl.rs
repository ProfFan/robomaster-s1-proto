//! The RM-S1 Gimbal Set Degree command

use crate::{
    duss::{cmd_set_rm::RMCommandType, cmd_set_types::CommandSetType},
    wire::RMWireFrameView,
};
use num_enum::TryFromPrimitive;

// gimbal_coodrdinate_ned = 0x00
// gimbal_coodrdinate_cur = 0x01
// gimbal_coodrdinate_car = 0x02  # pitch offset
// gimbal_coodrdinate_3 = 0x03  # pitch ned
// gimbal_coodrdinate_4 = 0x04  # yaw car, pitch ned
// gimbal_coodrdinate_5 = 0x05  # yaw car, pitch offset
/// Gimbal Coordinate Frames
#[derive(Debug, TryFromPrimitive, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum GimbalCoordinateFrame {
    NED = 0x00,
    CUR = 0x01,
    CAR = 0x02,
    Three = 0x03,
    Four = 0x04,
    Five = 0x05,
    Unknown = 0xFF,
}

/// The RM-S1 Gimbal Set Degree command
///
/// Sets the pitch and yaw degrees of the gimbal with a given acceleration
#[derive(Debug)]
pub struct GimbalSetDegreeView<T: AsRef<[u8]>> {
    pub packet: RMWireFrameView<T>,
}

impl<T: AsRef<[u8]>> GimbalSetDegreeView<T> {
    pub fn new(packet: RMWireFrameView<T>) -> Self {
        Self { packet }
    }

    pub fn task_id(&self) -> u8 {
        self.packet.payload()[0]
    }

    pub fn task_ctrl(&self) -> u8 {
        self.packet.payload()[1]
    }

    pub fn ctrl(&self) -> u8 {
        self.packet.payload()[2]
    }

    pub fn yaw_degree(&self) -> i16 {
        i16::from_le_bytes([self.packet.payload()[3], self.packet.payload()[4]])
    }

    pub fn roll_degree(&self) -> i16 {
        i16::from_le_bytes([self.packet.payload()[5], self.packet.payload()[6]])
    }

    pub fn pitch_degree(&self) -> i16 {
        i16::from_le_bytes([self.packet.payload()[7], self.packet.payload()[8]])
    }

    pub fn deviation(&self) -> i16 {
        i16::from_le_bytes([self.packet.payload()[9], self.packet.payload()[10]])
    }

    pub fn yaw_accel(&self) -> u16 {
        u16::from_le_bytes([self.packet.payload()[11], self.packet.payload()[12]])
    }

    pub fn roll_accel(&self) -> u16 {
        u16::from_le_bytes([self.packet.payload()[13], self.packet.payload()[14]])
    }

    pub fn pitch_accel(&self) -> u16 {
        u16::from_le_bytes([self.packet.payload()[15], self.packet.payload()[16]])
    }

    pub fn is_valid(&self) -> bool {
        self.packet.is_valid()
            && self.packet.cmd_set() == CommandSetType::RM as u8
            && self.packet.cmd_id() == RMCommandType::GIMBAL_DEGREE_SET as u8
            && self.packet.payload().len() == 17
    }
}

impl<T: AsRef<[u8]> + AsMut<[u8]>> GimbalSetDegreeView<T> {
    pub fn set_task_id(&mut self, task_id: u8) {
        self.packet.payload_mut()[0] = task_id;
    }

    pub fn set_task_ctrl(&mut self, task_ctrl: u8) {
        self.packet.payload_mut()[1] = task_ctrl;
    }

    pub fn set_ctrl(&mut self, ctrl: u8) {
        self.packet.payload_mut()[2] = ctrl;
    }

    pub fn set_yaw_degree(&mut self, yaw_degree: i16) {
        self.packet.payload_mut()[3..5].copy_from_slice(&yaw_degree.to_le_bytes());
    }

    pub fn set_roll_degree(&mut self, roll_degree: i16) {
        self.packet.payload_mut()[5..7].copy_from_slice(&roll_degree.to_le_bytes());
    }

    pub fn set_pitch_degree(&mut self, pitch_degree: i16) {
        self.packet.payload_mut()[7..9].copy_from_slice(&pitch_degree.to_le_bytes());
    }

    pub fn set_deviation(&mut self, deviation: i16) {
        self.packet.payload_mut()[9..11].copy_from_slice(&deviation.to_le_bytes());
    }

    pub fn set_yaw_accel(&mut self, yaw_accel: u16) {
        self.packet.payload_mut()[11..13].copy_from_slice(&yaw_accel.to_le_bytes());
    }

    pub fn set_roll_accel(&mut self, roll_accel: u16) {
        self.packet.payload_mut()[13..15].copy_from_slice(&roll_accel.to_le_bytes());
    }

    pub fn set_pitch_accel(&mut self, pitch_accel: u16) {
        self.packet.payload_mut()[15..17].copy_from_slice(&pitch_accel.to_le_bytes());
    }

    pub fn packet_mut(&mut self) -> &mut RMWireFrameView<T> {
        &mut self.packet
    }
}
