/// DUML Command Sets
use num_enum::TryFromPrimitive;

#[allow(non_camel_case_types)]
#[repr(u8)]
#[derive(Debug, Clone, Copy, TryFromPrimitive)]
pub enum CommandSetType {
    COMMON = 0,
    SPECIAL = 1,
    CAMERA = 2,
    FC = 3,
    GIMBAL = 4,
    CENTER = 5,
    RC = 6,
    WIFI = 7,
    DM368 = 8,
    HDVT = 9,
    VISION = 10,
    SIM = 11,
    ESC = 12,
    SMART_BATTERY = 13,
    HDVT_1765_GND = 14,
    S_TO_P_AIR = 15,
    S_TO_P_GND = 16,
    ADSB = 17,
    BVISION = 18,
    FPGA_AIR = 19,
    FPGA_GND = 20,
    GLASS = 21,
    MAVLINK = 22,
    WATCH = 23,
    PERCEPTION = 36,
    ROBOTIC_ARM = 51,
    RM = 0x3F,
    VIRTUAL_BUS = 0x48,
    MAX = 33,
}
