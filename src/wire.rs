//! The RM-S1 Wire Format
//!
//! This format closely resembles the DJI DUML/DUSS format, which is a multi-stream protocol
//! that uses CAN bus as the physical layer.
//!
//! Each frame has the following structure:
//! - SOF (Start of Frame) byte: 0x55
//! - low byte of the packet length
//! - 0b00000100 (0x4) & high 2 bits of the packet length
//! - Header CRC8
//! - Sender ID (1 byte)
//! - Receiver ID (1 byte)
//! - Low byte of the packet sequence number
//! - High byte of the packet sequence number
//! - 0bX0X00000, bit 8 = IS_ACK, bit 6 = NEED_ACK
//! - CMD_SET (1 byte)
//! - CMD_ID (1 byte)
//! - Payload (variable length)
//! - Payload CRC16 (2 bytes)

use num_enum::TryFromPrimitive;

#[derive(Debug, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum EncryptType {
    NO_ENC = 0x00,
    AES_128_ENC = 0x01,
    CUSTOM_ENC = 0x02,
    XOR_ENC = 0x03,
    DES_56_ENC = 0x04,
    DES_112_ENC = 0x05,
    AES_192_ENC = 0x06,
    AES_256_ENC = 0x07,
}

#[derive(PartialEq, Eq, Clone)]
pub struct RMWireFrameView<T: AsRef<[u8]>> {
    buf: T,
}

impl<T: AsRef<[u8]>> RMWireFrameView<T> {
    pub fn new(buf: T) -> RMWireFrameView<T> {
        RMWireFrameView { buf }
    }

    pub fn is_valid(&self) -> bool {
        let buffer = self.buf.as_ref();

        if buffer.len() < 13 {
            return false;
        }
        if buffer[0] != 0x55 {
            return false;
        }
        if buffer.len() < self.packet_length_field() as usize {
            return false;
        }
        if self.header_crc8() != crate::crc::rm_s1_crc8(&buffer[0..3]) {
            return false;
        }
        if self.packet_crc16_field() != self.crc16_computed() {
            return false;
        }

        true
    }

    pub fn header_crc8(&self) -> u8 {
        let buffer = self.buf.as_ref();
        buffer[3]
    }

    pub fn sender_id(&self) -> u8 {
        let buffer = self.buf.as_ref();
        buffer[4]
    }

    pub fn receiver_id(&self) -> u8 {
        let buffer = self.buf.as_ref();
        buffer[5]
    }

    pub fn packet_length_field(&self) -> u16 {
        let buffer = self.buf.as_ref();

        u16::from_le_bytes([buffer[1], buffer[2] & 0b0000_0011])
    }

    pub fn sequence_number(&self) -> u16 {
        let buffer = self.buf.as_ref();
        u16::from_le_bytes([buffer[6], buffer[7]])
    }

    pub fn is_ack(&self) -> bool {
        let buffer = self.buf.as_ref();
        buffer[8] & 0b1000_0000 != 0
    }

    pub fn need_ack(&self) -> bool {
        let buffer = self.buf.as_ref();
        buffer[8] & 0b0010_0000 != 0
    }

    pub fn encrypt_type(&self) -> EncryptType {
        let buffer = self.buf.as_ref();
        EncryptType::try_from(buffer[8] & 0b0000_0111).unwrap()
    }

    pub fn cmd_set(&self) -> u8 {
        let buffer = self.buf.as_ref();
        buffer[9]
    }

    pub fn cmd_id(&self) -> u8 {
        let buffer = self.buf.as_ref();
        buffer[10]
    }

    pub fn payload(&self) -> &[u8] {
        let buffer = self.buf.as_ref();
        &buffer[11..buffer.len() - 2]
    }

    pub fn packet_crc16_field(&self) -> u16 {
        let buffer = self.buf.as_ref();
        u16::from_le_bytes([buffer[buffer.len() - 2], buffer[buffer.len() - 1]])
    }

    pub fn crc16_computed(&self) -> u16 {
        let buffer = self.buf.as_ref();
        crate::crc::rm_s1_crc16(&buffer[0..buffer.len() - 2])
    }

    pub fn crc8_computed(&self) -> u8 {
        let buffer = self.buf.as_ref();
        crate::crc::rm_s1_crc8(&buffer[0..3])
    }
}

/// Debug formatter for RMWireFrameView
impl<T: AsRef<[u8]>> core::fmt::Debug for RMWireFrameView<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RMWireFrameView")
            .field("sender_id", &self.sender_id())
            .field("receiver_id", &self.receiver_id())
            .field("packet_length_field", &self.packet_length_field())
            .field("sequence_number", &self.sequence_number())
            .field("is_ack", &self.is_ack())
            .field("need_ack", &self.need_ack())
            .field("cmd_set", &self.cmd_set())
            .field("cmd_id", &self.cmd_id())
            .field("payload", &self.payload())
            .field("packet_crc16_field", &self.packet_crc16_field())
            .finish()
    }
}

impl<T: AsRef<[u8]> + AsMut<[u8]>> RMWireFrameView<T> {
    pub fn set_sender_id(&mut self, sender_id: u8) {
        let buffer = self.buf.as_mut();
        buffer[4] = sender_id;
    }

    pub fn set_receiver_id(&mut self, receiver_id: u8) {
        let buffer = self.buf.as_mut();
        buffer[5] = receiver_id;
    }

    pub fn set_sequence_number(&mut self, sequence_number: u16) {
        let buffer = self.buf.as_mut();
        buffer[6..8].copy_from_slice(&sequence_number.to_le_bytes());
    }

    pub fn set_is_ack(&mut self, is_ack: bool) {
        let buffer = self.buf.as_mut();
        if is_ack {
            buffer[8] |= 0b1000_0000;
        } else {
            buffer[8] &= 0b0111_1111;
        }
    }

    pub fn set_need_ack(&mut self, need_ack: bool) {
        let buffer = self.buf.as_mut();
        if need_ack {
            buffer[8] |= 0b0010_0000;
        } else {
            buffer[8] &= 0b1101_1111;
        }
    }

    pub fn set_encrypt_type(&mut self, encrypt_type: EncryptType) {
        let buffer = self.buf.as_mut();
        buffer[8] = (buffer[8] & 0b1111_1000) | (encrypt_type as u8);
    }

    pub fn set_cmd_set(&mut self, cmd_set: u8) {
        let buffer = self.buf.as_mut();
        buffer[9] = cmd_set;
    }

    pub fn set_cmd_id(&mut self, cmd_id: u8) {
        let buffer = self.buf.as_mut();
        buffer[10] = cmd_id;
    }

    pub fn set_payload(&mut self, payload: &[u8]) {
        let buffer = self.buf.as_mut();
        buffer[11..11 + payload.len()].copy_from_slice(payload);
    }

    pub fn set_packet_crc16_field(&mut self, packet_crc16_field: u16) {
        let buffer = self.buf.as_mut();
        let length = buffer.len();
        buffer[length - 2..].copy_from_slice(&packet_crc16_field.to_le_bytes());
    }

    pub fn set_header_crc8(&mut self) {
        let buffer = self.buf.as_mut();
        buffer[3] = crate::crc::rm_s1_crc8(&buffer[0..3]);
    }

    pub fn payload_mut(&mut self) -> &mut [u8] {
        let buffer = self.buf.as_mut();
        let payload_end = buffer.len() - 2;
        &mut buffer[11..payload_end]
    }
}

#[cfg(test)]
mod tests {
    extern crate std;
    use super::*;

    #[test]
    fn test_rm_wire_frame_view() {
        let buf: [u8; 36] = [
            0x55, 0x24, 0x04, 0x40, 0x58, 0x1d, 0x00, 0x00, //
            0x00, 0x00, 0xf0, 0x00, 0x6d, 0x69, 0x63, 0x3a, //
            0x68, 0x7a, 0x20, 0x65, 0x72, 0x72, 0x2c, 0x76, //
            0x61, 0x6c, 0x75, 0x65, 0x3a, 0x35, 0x31, 0x31, //
            0x0d, 0x0a, 0x41, 0x04,
        ];
        let frame = RMWireFrameView::new(&buf);

        assert_eq!(frame.sender_id(), 0x58);
        assert_eq!(frame.receiver_id(), 0x1d);
        assert_eq!(frame.packet_length_field(), 0x24);
        assert_eq!(frame.sequence_number(), 0);
        assert_eq!(frame.is_ack(), false);
        assert_eq!(frame.need_ack(), false);
        assert_eq!(frame.cmd_set(), 0x00);
        assert_eq!(frame.cmd_id(), 0xf0);
        let expected_str = b"\0mic:hz err,value:511\r\n";
        assert_eq!(frame.payload(), expected_str);
    }

    #[test]
    fn test_rm_unknown_msg() {
        let buf = [
            0x55, 0x0E, 0x04, 0x66, 0x09, 0x03, 0x4E, 0x06, 0xA0, 0x48, 0x08, 0x01, 0xC2, 0xE8,
        ];

        let frame = RMWireFrameView::new(&buf);

        std::println!("{:#0X?}", frame);

        let crc16 = crate::crc::rm_crc16(0x3692, &buf[..buf.len() - 2]);
        assert_eq!(crc16, frame.packet_crc16_field());

        assert_eq!(frame.is_valid(), true);
    }

    #[test]
    fn test_heartbeat_msg() {
        // The so call heartbeat message is actually a RMC (ReMote Control) message
        let buf = [
            0x55, 0x1B, 0x04, 0x75, 0x09, 0xC3, 0xE0, 0x00, 0x00, 0x3F, 0x60, 0x00, 0x04, 0x20,
            0x00, 0x01, 0x00, 0x40, 0x00, 0x02, 0x10, 0x04, 0x03, 0x00, 0x04, 0xFA, 0xF0,
        ];
        // RM DBUS: 0x00, 0x04, 0x20, 0x00, 0x01, 0xD8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00
        // Payload: 0x00, 0x04, 0x20, 0x00, 0x01, 0x00, 0x40, 0x00, 0x02, 0x10, 0x04, 0x03, 0x00, 0x04, 0xFA, 0xF0,

        let frame = RMWireFrameView::new(&buf);

        assert_eq!(frame.sender_id(), 0x09);
        assert_eq!(frame.receiver_id(), 0xC3);
        assert_eq!(frame.packet_length_field(), 0x1B);
        assert_eq!(frame.sequence_number(), 0xE0);
        assert_eq!(frame.is_ack(), false);
        assert_eq!(frame.need_ack(), false);
        assert_eq!(
            frame.cmd_set(),
            crate::duss::cmd_set_types::CommandSetType::RM as u8
        );
        assert_eq!(
            frame.cmd_id(),
            crate::duss::cmd_set_rm::RMCommandType::FC_RMC as u8
        );
        assert_eq!(frame.payload(), &buf[11..buf.len() - 2]);

        let crc8_calculated = crate::crc::rm_crc8(0x77, &buf[..3]);
        assert_eq!(crc8_calculated, 0x75);

        let crc16_calculated = crate::crc::rm_crc16(0x3692, &buf[..buf.len() - 2]);
        assert_eq!(crc16_calculated, 0xF0FA);

        let my_payload_normal_mode = b"\x00\x04 \x00\x01\x08@\x00\x02\x10\x04\x00\x00\x04";
        assert_ne!(frame.payload(), my_payload_normal_mode);

        let payload = my_payload_normal_mode;
        // 11-bit Unsigned S-BUS RC data
        // Bit 0-10: Channel 0
        // Bit 11-21: Channel 1
        // Bit 22-32: Channel 2
        // Bit 33-43: Channel 3
        let rc_ch0 = payload[0] as u16 | ((payload[1] as u16) << 8) & 0x7FF;
        let rc_ch1 = payload[1] as u16 >> 3 | ((payload[2] as u16) << 5) & 0x7FF;
        let rc_ch2 = payload[2] as u16 >> 6
            | ((payload[3] as u16) << 2)
            | ((payload[4] as u16) << 10) & 0x7FF;
        let rc_ch3 = payload[4] as u16 >> 1 | ((payload[5] as u16) << 7) & 0x7FF;
        // Bit 44-54: Channel 4
        let rc_ang_z = (payload[5] & 0xF0) as u16 >> 4 | ((payload[6] as u16) << 4) & 0x7FF;
        // Bit 55-65: Channel 5
        let rc_ch5 = payload[6] as u16 >> 7
            | ((payload[7] as u16) << 1)
            | ((payload[8] as u16) << 9) & 0x7FF;
        // Bit 66-76: Channel 6
        let rc_ch6 = payload[8] as u16 >> 2 | ((payload[9] as u16) << 6) & 0x7FF;

        assert_eq!(rc_ch0, 1024);
        assert_eq!(rc_ch1, 1024);
        assert_eq!(rc_ch2, 1024);
        assert_eq!(rc_ch3, 1024);
        assert_eq!(rc_ang_z, 1024);
        assert_eq!(rc_ch5, 1024);
        assert_eq!(rc_ch6, 1024);
    }

    #[test]
    fn test_slow_mode_enter() {
        let buf = [
            0x55, 0x0E, 0x04, 0xFF, 0x09, 0xC3, 0xFF, 0xFF, 0x40, 0x3F, 0x3F, 0x03, 0xFF, 0xFF,
        ];

        let frame = RMWireFrameView::new(&buf);

        assert_eq!(frame.sender_id(), 0x09);
        assert_eq!(frame.receiver_id(), 0xC3);
        assert_eq!(frame.packet_length_field(), 0x0E);

        assert_eq!(frame.sequence_number(), 0xFFFF);
        assert_eq!(
            frame.cmd_set(),
            crate::duss::cmd_set_types::CommandSetType::RM as u8
        );
        assert_eq!(
            frame.cmd_id(),
            crate::duss::cmd_set_rm::RMCommandType::SET_CHASSIS_SPEED as u8
        );
        assert_eq!(frame.payload(), &[0x3]);
        assert_eq!(frame.need_ack(), false);
    }

    #[test]
    fn test_set_led_1() {
        let buf = [
            0x55, 0x1A, 0x04, 0xFF, 0x09, 0x18, 0xFF, 0xFF, 0x00, 0x3F, 0x32, 0x01, 0xFF, 0x00,
            0x00, 0x00, 0xFF, 0x00, 0xC8, 0x00, 0xC8, 0x00, 0x0F, 0x00, 0xFF, 0xFF,
        ];

        let frame = RMWireFrameView::new(&buf);

        assert_eq!(frame.sender_id(), 0x09);
        assert_eq!(frame.receiver_id(), 0x18);

        assert_eq!(frame.packet_length_field(), 0x1A);
        assert_eq!(frame.sequence_number(), 0xFFFF);
        assert_eq!(
            frame.cmd_set(),
            crate::duss::cmd_set_types::CommandSetType::RM as u8
        );
        assert_eq!(
            frame.cmd_id(),
            crate::duss::cmd_set_rm::RMCommandType::ARMOR_LED_SET as u8
        );
        assert_eq!(
            frame.payload(),
            &[0x01, 0xFF, 0x00, 0x00, 0x00, 0xFF, 0x00, 0xC8, 0x00, 0xC8, 0x00, 0x0F, 0x00]
        );
    }

    #[test]
    fn test_shoot_gel_gun() {
        let buf = [
            0x55, 0x0E, 0x04, 0xFF, 0x09, 0x17, 0xFF, 0xFF, 0x00, 0x3F, 0x51, 0x01, 0xFF, 0xFF,
        ];
        let frame = RMWireFrameView::new(&buf);

        assert_eq!(frame.sender_id(), 0x09);
        assert_eq!(frame.receiver_id(), 0x17);
        assert_eq!(frame.packet_length_field(), 0x0E);

        assert_eq!(
            frame.cmd_set(),
            crate::duss::cmd_set_types::CommandSetType::RM as u8
        );
        assert_eq!(
            frame.cmd_id(),
            crate::duss::cmd_set_rm::RMCommandType::SHOOT_CMD as u8
        );
        assert_eq!(frame.payload(), &[0x01]);
    }

    #[test]
    fn test_gimbal_set_angle_robostack() {
        let buf = [
            0x55, 0x14, 0x04, 0xFF, 0x09, 0x04, 0xFF, 0xFF, 0x00, 0x04, 0x69, 0x08, 0x05, 0x00,
            0x00, 0x00, 0x00, 0x6D, 0xFF, 0xFF,
        ];

        let frame = RMWireFrameView::new(&buf);

        assert_eq!(frame.sender_id(), 0x09);
        assert_eq!(frame.receiver_id(), 0x04);
        assert_eq!(frame.packet_length_field(), 0x14);
        assert_eq!(
            frame.cmd_set(),
            crate::duss::cmd_set_types::CommandSetType::GIMBAL as u8
        );
        assert_eq!(frame.cmd_id(), 0x69);

        std::println!("{:0x?}", frame.payload())
    }

    #[test]
    fn test_rm_wire_frame_view_debug() {
        let buf: [u8; 36] = [
            0x55, 0x24, 0x04, 0x40, 0x58, 0x1d, 0x00, 0x00, //
            0x00, 0x00, 0xf0, 0x00, 0x6d, 0x69, 0x63, 0x3a, //
            0x68, 0x7a, 0x20, 0x65, 0x72, 0x72, 0x2c, 0x76, //
            0x61, 0x6c, 0x75, 0x65, 0x3a, 0x35, 0x31, 0x31, //
            0x0d, 0x0a, 0x41, 0x04,
        ];
        let frame = RMWireFrameView::new(&buf);

        assert_eq!(
            std::format!("{:#0X?}", frame),
            r#"RMWireFrameView {
    sender_id: 0x58,
    receiver_id: 0x1D,
    packet_length_field: 0x24,
    sequence_number: 0x0,
    is_ack: false,
    need_ack: false,
    cmd_set: 0x0,
    cmd_id: 0xF0,
    payload: [
        0x0,
        0x6D,
        0x69,
        0x63,
        0x3A,
        0x68,
        0x7A,
        0x20,
        0x65,
        0x72,
        0x72,
        0x2C,
        0x76,
        0x61,
        0x6C,
        0x75,
        0x65,
        0x3A,
        0x35,
        0x31,
        0x31,
        0xD,
        0xA,
    ],
    packet_crc16_field: 0x441,
}"#
        );
    }
}
