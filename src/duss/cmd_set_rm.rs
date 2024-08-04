#[allow(non_camel_case_types)]
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum RMCommandType {
    HIT_EVENT = 0x02,
    SPECIAL_CONTROL = 0x04,
    WATER_GUN_PARM_SET = 0x05,
    ARMOR_VOICE_PARAMS_SET = 0x07,
    GAME_STATE_SYNC = 0x09,
    GAMECTRL_CMD = 0x0a,
    GAME_GROUP_CONFIG = 0x0b,
    GAME_START_END_CONFIG = 0x0c,
    SKILL_SEND = 0x0f,
    IR_EVENT = 0x10,
    BLOOD_LED_SET = 0x11,
    MODULE_STATUS_PUSH = 0x12,
    WORK_MODE_SET = 0x19,
    PLAY_SOUND = 0x1a,
    SET_SPEAKER_VOLUME = 0x1b,
    GET_SPEAKER_VOLUME = 0x1c,
    AUDIO_TO_APP = 0x1d,
    SET_AUDIO_STATUS = 0x1e,
    WHEEL_SPEED_SET = 0x20,
    SPEED_SET = 0x21,
    FOLLOW_MODE_SET = 0x22,
    FPV_MODE_SPEED_SET = 0x23,
    GROUND_MODE_SET = 0x24,
    POSITION_SET = 0x25,
    WHEEL_STATUS_SET = 0x26,
    WHEEL_STATUS_GET = 0x27,
    SPEED_MODE_SET = 0x28,
    CHASSIS_POSITION_TASK_PUSH = 0x2a,
    SET_CHASSIS_PWM_FREQ = 0x2b,
    GET_CHASSIS_PWM_FREQ = 0x2d,
    ARMOR_GET_STATE = 0x31,
    ARMOR_LED_SET = 0x32,
    LED_COLOR_SET = 0x33,
    SET_CHASSIS_PWM_VALUE = 0x3c,
    GET_CHASSIS_PWM_VALUE = 0x3d,
    SET_TANK_WORK_MODE = 0x46,
    GET_TANK_WORK_MODE = 0x47,
    EXIT_LOW_POWER_MODE = 0x4c,
    SHOOT_EVENT = 0x50,
    SHOOT_CMD = 0x51,
    SHOOT_GET_STATE = 0x52,
    SHOOT_MODE_SET = 0x53,
    SHOOT_MODE_GET = 0x54,
    GUN_LED_SET = 0x55,
    FC_RMC = 0x60,
    FC_GET_STATE = 0x61,
    SCRIPT_DOWNLOAD_DATA = 0xA1,
    SCRIPT_DOWNLOAD_FINSH = 0xA2,
    SCRIPT_CTRL = 0xA3,
    SCRIPT_CUSTOM_INFO_PUSH = 0xA4,
    SCRIPT_BLOCK_STATUS_PUSH = 0xA5,
    SCRIPT_PARAMS_INFO_PUSH = 0xA6,
    SCRIPT_LOG_INFO = 0xA7,
    CUSTOM_SKILL_CONFIG_QUERY = 0xA8,
    SCRIPT_LOCAL_SUB_SERVICE = 0xA9,
    SUB_MOBILE_INFO = 0xAB,
    MOBILE_INFO_PUSH = 0xAC,
    SCRATCH_AUTO_TEST = 0xAF,
    GIMBAL_DEGREE_SET = 0xB0,
    GIMBAL_POSITION_TASK_PUSH = 0xB1,
    GIMBAL_RESET_POSITION_SET = 0xB2,
    PLAY_SOUND_TASK = 0xB3,
    PLAY_SOUND_TASK_PUSH = 0xB4,
    ROBOTIC_ARM_POSITION_TASK_SET = 0xB5,
    ROBOTIC_ARM_POSITION_TASK_PUSH = 0xB6,
    SERVO_ANGLE_TASK_SET = 0xB7,
    SERVO_ANGLE_TASK_PUSH = 0xB8,
    CUSTOM_UI_ATTRIBUTE_SET = 0xBA,
    CUSTOM_UI_ACTION_TRIGGER = 0xBB,
    CUSTOM_SOUND_CONVERT = 0xBC,
    LINK_STATE_PUSH = 0xD0,
    SDK_MODE_SET = 0xD1,
    STREAM_CTRL = 0xD2,
    UART_CONFIG = 0xC0,
    UART_MSG = 0xC1,
    UART_STATUS_PUSH = 0xC2,
    MEDIA_SOUND_RECOGNIZE_SET = 0xE3,
    MEDIA_SOUND_RECOGNIZE_PUSH = 0xE4,
    MEDIA_CAMERA_BRIGHTNESS_GET = 0xE5,
    GET_SENSOR_ADAPTER_DATA = 0xF0,
    SET_SENSOR_ADAPTER_PARAM = 0xF1,
    GET_SENSOR_ADAPTER_PARAM = 0xF2,
    PUSH_SENSOR_ADAPTER_IO_EVENT = 0xF3,
    PUSH_SENSOR_ADAPTER_ADC_VALUE = 0xF4,
    PRODUCT_ATTRIBUTE_GET = 0xFE,
}
