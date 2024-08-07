use num_enum::TryFromPrimitive;

#[allow(non_camel_case_types)]
#[repr(u8)]
#[derive(Debug, Clone, Copy, TryFromPrimitive)]
pub enum GimbalCommandType {
    GIMBAL_RESERVED = 0x00,
    GIMBAL_CONTROL = 0x01,
    GIMBAL_GET_POSITION = 0x02,
    GIMBAL_SET_PARAM = 0x03,
    GIMBAL_GET_PARAM = 0x04,
    GIMBAL_PUSH_POSITION = 0x05,
    GIMBAL_PUSH_AETR = 0x06,
    GIMBAL_ADJUST_ROLL = 0x07,
    GIMBAL_CALIBRATION = 0x08,
    GIMBAL_RESERVED2 = 0x09,
    GIMBAL_EXT_CTRL_DEGREE = 0x0A,
    GIMBAL_GET_EXT_CTRL_STATUS = 0x0B,
    GIMBAL_EXT_CTRL_ACCEL = 0x0C,
    GIMBAL_SUSPEND_RESUME = 0x0D,
    GIMBAL_THIRDP_MAGN = 0x0E,
    GIMBAL_SET_USER_PARAM = 0x0F,
    GIMBAL_GET_USER_PARAM = 0x10,
    GIMBAL_SAVE_USER_PARAM = 0x11,
    GIMBAL_RESUME_DEFAULT_PARAM = 0x13,
    GIMBAL_PUSH_TYPE = 0x1C,
    GIMBAL_DEGREE_INFO_SUBSCRIPTION = 0x1E,
    GIMBAL_LOCK = 0x39,
    GIMBAL_ROTATE_CAMERA_X_AXIS = 0x3A,
    GIMBAL_GET_TEMP = 0x45,
    GIMBAL_SET_MODE = 0x4C,
    GIMBAL_ROTATE_EXP_CMD = 0x68,

    // 0x69 from RoboStackS1, set gimbal angle
    GIMBAL_SET_ANGLES = 0x69,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{duss::cmd_set_types::CommandSetType, wire::RMWireFrameView};

    #[test]

    fn test_gimbal_command() {
        let cmd = [
            0x55, 0x14, 0x04, 0xFF, 0x09, 0x04, 0xFF, 0xFF, 0x00, 0x04, 0x69, 0x08, 0x05, 0x00,
            0x00, 0x00, 0x00, 0x6D, 0xFF, 0xFF,
        ];

        let result = RMWireFrameView::new(cmd);

        assert_eq!(result.sender_id(), 0x09);
        assert_eq!(result.receiver_id(), 0x04);
        assert_eq!(result.cmd_set(), CommandSetType::GIMBAL as u8);
        assert_eq!(result.cmd_id(), GimbalCommandType::GIMBAL_SET_ANGLES as u8);
    }
}
