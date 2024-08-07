use num_enum::TryFromPrimitive;

#[allow(non_camel_case_types)]
#[repr(u16)]
#[derive(Debug, Clone, Copy, TryFromPrimitive)]
pub enum RMS1Address {
    camera_id = 100,
    mobile_id = 200,
    chassis_id = 306,
    gimbal_id = 400,
    gun_id = 2300,
    vision_id = 1707,
    battery_id = 1100,
    hdvt_uav_id = 900,
    system_id = 801,
    system_scratch_id = 803,
    scratch_sys_id = 905,
    scratch_script_id = 906,
    armor_id = 2400,
    armor1_id = 2401,
    armor2_id = 2402,
    armor3_id = 2403,
    armor4_id = 2404,
    armor5_id = 2405,
    armor6_id = 2406,
    esc0_id = 1200,
    esc1_id = 1201,
    esc2_id = 1202,
    esc3_id = 1203,
    blackbox_id = 2900,
    sensor_adapter_id = 2200,
    sensor_adapter1_id = 2201,
    sensor_adapter2_id = 2202,
    sensor_adapter3_id = 2203,
    sensor_adapter4_id = 2204,
    sensor_adapter5_id = 2205,
    sensor_adapter6_id = 2206,
    sensor_adapter7_id = 2207,
    tof_id = 1800,
    tof1_id = 1801,
    tof2_id = 1802,
    tof3_id = 1803,
    tof4_id = 1804,
    servo_id = 2500,
    servo1_id = 2501,
    servo2_id = 2502,
    servo3_id = 2503,
    servo4_id = 2504,
    robotic_gripper_id = 2701,
    robotic_arm_id = 2702,
}

/// HostID to PackID
///
/// ```python
/// def hostid2packid(host_id):
///     host_id = ((int(host_id / 100) & 0x1f) | ((host_id % 100) << 5) & 0xe0)
///     return [host_id]
/// ```
pub fn hostid2packid(host_id: u16) -> u8 {
    ((host_id / 100) & 0x1f | ((host_id % 100) << 5) & 0xe0) as u8
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hostid2packid() {
        // CAMERA = 100
        assert_eq!(hostid2packid(100), 0x01);
        // MOBILE = 200
        assert_eq!(hostid2packid(200), 0x02);
        // CHASSIS = 306
        assert_eq!(hostid2packid(306), 0xC3);
        // GIMBAL = 400
        assert_eq!(hostid2packid(400), 0x04);
        // GUN = 2300
        assert_eq!(hostid2packid(2300), 0x17);
        // VISION = 1707
        assert_eq!(hostid2packid(1707), 0xF1);
        // BATTERY = 1100
        assert_eq!(hostid2packid(1100), 0x0B);
        // HDVT_UAV = 900
        assert_eq!(hostid2packid(900), 0x09);
        // SYSTEM = 801
        assert_eq!(hostid2packid(801), 0x28);
        // SYSTEM_SCRATCH = 803
        assert_eq!(hostid2packid(803), 0x68);
        // SCRATCH_SYS = 905
        assert_eq!(hostid2packid(905), 0xA9);
        // SCRATCH_SCRIPT = 906
        assert_eq!(hostid2packid(906), 0xC9);
        // ARMOR = 2400
        assert_eq!(hostid2packid(2400), 0x18);
        // ARMOR1 = 2401
        assert_eq!(hostid2packid(2401), 0x38);
        // ESC0 = 1200
        assert_eq!(hostid2packid(1200), 0x0C);
        // ESC1 = 1201
        assert_eq!(hostid2packid(1201), 0x2C);
        // ESC2 = 1202
        assert_eq!(hostid2packid(1202), 0x4C);
    }
}
