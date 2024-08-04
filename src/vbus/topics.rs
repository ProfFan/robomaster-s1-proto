//! This file contains the topics that are used in the DDS communication between the FC and the HDVT.
//!
//! # DDS Topics (FC -> HDVT)
//!
//! These topics are on the CAN bus and are used to send data from the FC to the HDVT.
//! High 32-bits of these UUIDs are 0x20003 (0x03, 0x00, 0x02, 0x00). 0x03 is the address of the FC.
//!
//! # SDK Topics (HDVT -> SDK)
//!
//! These topics are available in the Python SDK code (open source). However, we cannot use these topics in the low-level CAN bus communication.
//! One must first enter SDK mode to receive these topics. These topics are for the SDK to receive data from the HDVT.
//! Most likely, we cannot use these on the CAN bus.
//!
//! Low 32-bits:
//! - 0xc14cb7c5,  # esc_info
//! - 0xeeb7cece,  # ns_pos
//! - 0x49a4009c,  # ns_vel
//! - 0xa7985b8d,  # ns_imu
//! - 0x6b986306,  # attitude_info
//! - 0x4a2c6d55,  # ns_sa_status
//! - 0xf79b3c97,  # gimbal_pos
//! - 0x55e9a0fa,  # stick_flag
//! - 0x5f0059e7,  # servo_id_in_roboticarm_mode
//! - 0x6862229f,  # battery_info
//!
//! High 32-bits:
//! - 0x20009 (0x09, 0x00, 0x02, 0x00)
//! - This is different from the CAN bus topics where the low 32-bits are 0x20003 (0x03, 0x00, 0x02, 0x00)
//! - Most likely, 0x09 means this topic UUID is on the HDVT
//! - Similarly, 0x03 means this topic UUID is on the FC

use bytemuck::{Pod, Zeroable};

#[derive(Debug, Clone, Copy, Pod, Zeroable)]
#[repr(C)]
pub struct DdsUid {
    pub uid: [u8; 8],
}

pub const DDS_TURRET_YAW: [u8; 8] = [0xa7, 0x02, 0x29, 0x88, 0x03, 0x00, 0x02, 0x00]; // Turret Yaw
pub const DDS_CHASSIS_VELOCITY: [u8; 8] = [0x66, 0x3e, 0x3e, 0x4c, 0x03, 0x00, 0x02, 0x00]; // Velocity
pub const DDS_BATTERY_STATS: [u8; 8] = [0xfb, 0xdc, 0xf5, 0xd7, 0x03, 0x00, 0x02, 0x00]; // Battery
pub const DDS_ESC_STATE: [u8; 8] = [0x09, 0xa3, 0x26, 0xe2, 0x03, 0x00, 0x02, 0x00]; // ESC state
pub const DDS_IMU_DATA: [u8; 8] = [0xf4, 0x1d, 0x1c, 0xdc, 0x03, 0x00, 0x02, 0x00]; // IMU data
pub const DDS_TURRET_ATTITUDE_RPY: [u8; 8] = [0x42, 0xee, 0x13, 0x1d, 0x03, 0x00, 0x02, 0x00]; // Attitude (RPY)
pub const DDS_BASE_POSITION: [u8; 8] = [0xb3, 0xf7, 0xe6, 0x47, 0x03, 0x00, 0x02, 0x00]; // Position

/// [9d, 1a, 1c, 99, 03, 00, 02, 00]
///
/// This is an unknown topic. Subscribed by 0x203 along with DDS_BATTERY_STATS.
pub const DDS_UNK_1: [u8; 8] = [0x9d, 0x1a, 0x1c, 0x99, 0x03, 0x00, 0x02, 0x00];
