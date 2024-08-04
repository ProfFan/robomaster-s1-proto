use crate::wire::RMWireFrameView;
use bytemuck::{self};

use super::topics::DdsUid;

/// Published Topic Packet
pub struct RMTopicView<T: AsRef<[u8]>> {
    pub packet: RMWireFrameView<T>,
}

impl<T: AsRef<[u8]>> RMTopicView<T> {
    pub fn new(packet: RMWireFrameView<T>) -> RMTopicView<T> {
        RMTopicView { packet }
    }

    pub fn sub_mode(&self) -> u8 {
        self.packet.payload()[0]
    }

    pub fn sub_id(&self) -> u8 {
        self.packet.payload()[1]
    }

    pub fn data(&self) -> &[u8] {
        &self.packet.payload()[2..]
    }
}

/// Subscriber Add Packet
///
/// Add a subscriber to topics.
///
/// # Example
///
/// For subscribing to the yaw/pitch/roll topic (UID 0x42, 0xee, 0x13, 0x1d, 0x03, 0x00, 0x02, 0x00).
/// We send a packet to the subcontroller with the following payload:
/// - Header: CMDSET_DDS, CMDID_DDS_ADD_SUB
/// - Payload:
///     - My node ID (Base, 0x09)
///     - My stream ID (any unused stream ID)
///     - Flags (0x3)
///     - Sub mode (0x0)
///     - Number of topics (0x1)
///     - Array of topics
///     - Frequency of the topic (2 bytes)
pub struct RMAddSubView<T: AsRef<[u8]>> {
    pub packet: RMWireFrameView<T>,
}

impl<T: AsRef<[u8]>> RMAddSubView<T> {
    pub fn new(packet: RMWireFrameView<T>) -> RMAddSubView<T> {
        RMAddSubView { packet }
    }

    pub fn sub_node_id(&self) -> u8 {
        self.packet.payload()[0]
    }

    pub fn sub_stream_id(&self) -> u8 {
        self.packet.payload()[1]
    }

    pub fn timestamp_requested(&self) -> bool {
        self.packet.payload()[2] & 0b1 != 0
    }

    pub fn stop_when_disconnected(&self) -> bool {
        self.packet.payload()[2] & 0b10 != 0
    }

    pub fn sub_mode(&self) -> u8 {
        self.packet.payload()[3]
    }

    pub fn num_topics(&self) -> u8 {
        self.packet.payload()[4]
    }

    pub fn topics(&self) -> Option<&[DdsUid]> {
        let payload = self.packet.payload();
        bytemuck::try_cast_slice(&payload[5..payload.len() - 2]).ok()
    }

    pub fn frequency(&self) -> u16 {
        let payload = self.packet.payload();
        u16::from_le_bytes([payload[payload.len() - 2], payload[payload.len() - 1]])
    }
}

/// Mutable Methods
impl<T: AsRef<[u8]> + AsMut<[u8]>> RMAddSubView<T> {}

#[cfg(test)]
mod test {
    extern crate std;

    use crate::vbus::{CMDID_DDS_ADD_SUB, CMDSET_DDS};

    use super::*;

    #[test]
    fn test_topic_view() {
        // Chassis Odometry and Metrics
        // 0000   55 31 04 53 03 04 8d 04
        // 0000   20 48 08 00 00 99 1d 00
        // 0000   00 f0 a0 73 00 00 00 00
        // 0000   00 00 00 00 00 00 00 00
        // 0000   00 00 00 00 00 ac 2e dd
        // 0000   00 17 fd ff ff 54 00 3f
        // 0000   41 00 00 00 00 00 00 00
        let buf = [
            0x55, 0x31, 0x04, 0x53, 0x03, 0x04, 0x8d, 0x04, //
            0x20, 0x48, 0x08, 0x00, 0x00, 0x99, 0x1d, 0x00, //
            0x00, 0xf0, 0xa0, 0x73, 0x00, 0x00, 0x00, 0x00, //
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
            0x00, 0x00, 0x00, 0x00, 0x00, 0xac, 0x2e, 0xdd, //
            0x00, 0x17, 0xfd, 0xff, 0xff, 0x54, 0x00, 0x3f, //
            0x41,
        ];

        let packet = RMWireFrameView::new(&buf);
        let topic = RMTopicView::new(packet);

        assert_eq!(topic.packet.cmd_set(), crate::vbus::CMDSET_DDS);
        assert_eq!(topic.packet.is_valid(), true);
        assert_eq!(topic.sub_mode(), 0x00);
        assert_eq!(topic.sub_id(), 0x00); // Subscribe Session ID
        assert_eq!(
            0.0, // Odometry Quaternion W
            f32::from_le_bytes(topic.data()[8..12].try_into().unwrap())
        );
        assert_eq!(
            0.0, // Odometry Quaternion X
            f32::from_le_bytes(topic.data()[12..16].try_into().unwrap())
        );
        assert_eq!(
            0.0, // Odometry Quaternion Y
            f32::from_le_bytes(topic.data()[16..20].try_into().unwrap())
        );
        assert_eq!(
            0.0, // Odometry Quaternion Z
            f32::from_le_bytes(topic.data()[20..24].try_into().unwrap())
        );
        assert_eq!(
            11948, // Battery Voltage (mV)
            u16::from_le_bytes(topic.data()[24..26].try_into().unwrap())
        );
        assert_eq!(
            221, // Battery Temperature (0.1 C)
            u16::from_le_bytes(topic.data()[26..28].try_into().unwrap())
        );
        assert_eq!(
            -745, // Battery Current (mA), negative = output
            i32::from_le_bytes(topic.data()[28..32].try_into().unwrap())
        );
        assert_eq!(84, topic.data()[32]);
    }

    #[test]
    fn test_subscribe_add_parse() {
        // A sample subscribe add packet with 7 topics with 50Hz update rate
        // - Turret Yaw
        // - Velocity
        // - Battery
        // - ESC state
        // - IMU data
        // - Attitude (RPY)
        // - Position
        let buf: [u8; 76] = [
            0x55, 0x4c, 0x04, 0x6c, 0x09, 0x03, 0x8d, 0x04, 0x40, 0x48, 0x03, 0x09, 0x01, 0x03,
            0x00, 0x07, 0xa7, 0x02, 0x29, 0x88, 0x03, 0x00, 0x02, 0x00, 0x66, 0x3e, 0x3e, 0x4c,
            0x03, 0x00, 0x02, 0x00, 0xfb, 0xdc, 0xf5, 0xd7, 0x03, 0x00, 0x02, 0x00, 0x09, 0xa3,
            0x26, 0xe2, 0x03, 0x00, 0x02, 0x00, 0xf4, 0x1d, 0x1c, 0xdc, 0x03, 0x00, 0x02, 0x00,
            0x42, 0xee, 0x13, 0x1d, 0x03, 0x00, 0x02, 0x00, 0xb3, 0xf7, 0xe6, 0x47, 0x03, 0x00,
            0x02, 0x00, 0x32, 0x00, 0x54, 0xbb,
        ];

        let packet = RMWireFrameView::new(&buf);
        let addsub_view = RMAddSubView::new(packet);

        assert_eq!(addsub_view.packet.cmd_set(), crate::vbus::CMDSET_DDS);
        assert_eq!(addsub_view.packet.cmd_id(), crate::vbus::CMDID_DDS_ADD_SUB);

        assert_eq!(addsub_view.packet.is_valid(), true);

        assert_eq!(addsub_view.sub_node_id(), 0x09);
        assert_eq!(addsub_view.sub_mode(), 0x00);
        assert_eq!(addsub_view.sub_stream_id(), 0x01);

        assert_eq!(addsub_view.num_topics(), 7);
        assert_eq!(addsub_view.frequency(), 50);

        let topics = addsub_view.topics().unwrap();

        assert_eq!(
            topics[0].uid,
            [0xa7, 0x02, 0x29, 0x88, 0x03, 0x00, 0x02, 0x00] // Turret Yaw
        );
        assert_eq!(
            topics[1].uid,
            [0x66, 0x3e, 0x3e, 0x4c, 0x03, 0x00, 0x02, 0x00] // Velocity
        );
        assert_eq!(
            topics[2].uid,
            [0xfb, 0xdc, 0xf5, 0xd7, 0x03, 0x00, 0x02, 0x00] // Battery
        );
        assert_eq!(
            topics[3].uid,
            [0x09, 0xa3, 0x26, 0xe2, 0x03, 0x00, 0x02, 0x00] // ESC state
        );
        assert_eq!(
            topics[4].uid,
            [0xf4, 0x1d, 0x1c, 0xdc, 0x03, 0x00, 0x02, 0x00] // IMU data
        );
        assert_eq!(
            topics[5].uid,
            [0x42, 0xee, 0x13, 0x1d, 0x03, 0x00, 0x02, 0x00] // Attitude (RPY)
        );
        assert_eq!(
            topics[6].uid,
            [0xb3, 0xf7, 0xe6, 0x47, 0x03, 0x00, 0x02, 0x00] // Position
        );
    }

    #[test]
    fn test_robostack_init_commands() {
        let buf1 = [
            0x55, 0x12, 0x04, 0xFF, 0x09, 0x03, 0x01, 0x00, 0x40, 0x48, 0x01, 0x09, 0x00, 0x00,
            0x00, 0x03, 0xFF, 0xFF,
        ];
        let buf2 = [
            0x55, 0x1C, 0x04, 0xFF, 0x09, 0x03, 0x02, 0x00, 0x40, 0x48, 0x03, 0x09, 0x00, 0x03,
            0x00, 0x01, 0xFB, 0xDC, 0xF5, 0xD7, 0x03, 0x00, 0x02, 0x00, 0x01, 0x00, 0xFF, 0xFF,
        ];
        let buf3 = [
            0x55, 0x12, 0x04, 0xFF, 0x09, 0x03, 0x03, 0x00, 0x40, 0x48, 0x01, 0x09, 0x00, 0x00,
            0x00, 0x03, 0xFF, 0xFF,
        ];

        let buf4 = [
            0x55, 0x24, 0x04, 0xFF, 0x09, 0x03, 0x04, 0x00, 0x40, 0x48, 0x03, 0x09, 0x01, 0x03,
            0x00, 0x02, 0xA7, 0x02, 0x29, 0x88, 0x03, 0x00, 0x02, 0x00, 0x66, 0x3E, 0x3E, 0x4C,
            0x03, 0x00, 0x02, 0x00, 0x32, 0x00, 0xFF, 0xFF,
        ];

        let packet1 = RMWireFrameView::new(&buf1);
        let packet2 = RMWireFrameView::new(&buf2);
        let packet3 = RMWireFrameView::new(&buf3);
        let packet4 = RMWireFrameView::new(&buf4);
        std::println!("packet4: {:02x?}", packet4);

        let init2_view = RMAddSubView::new(packet2);
        let init4_view = RMAddSubView::new(packet4);

        assert_eq!(packet1.cmd_set(), CMDSET_DDS);
        assert_eq!(packet1.cmd_id(), 0x01);
        assert_eq!(packet1.packet_length_field(), 0x12);

        assert_eq!(init2_view.packet.cmd_set(), CMDSET_DDS);
        assert_eq!(init2_view.packet.cmd_id(), CMDID_DDS_ADD_SUB);
        assert_eq!(init2_view.num_topics(), 1);
        assert_eq!(init2_view.sub_node_id(), 0x09);
        assert_eq!(init2_view.sub_mode(), 0x00);
        assert_eq!(init2_view.sub_stream_id(), 0x00);
        assert_eq!(init2_view.frequency(), 0x01);

        let topics = init2_view.topics().unwrap();
        assert_eq!(
            topics[0].uid,
            [0xfb, 0xdc, 0xf5, 0xd7, 0x03, 0x00, 0x02, 0x00] // Battery
        );

        assert_eq!(packet3.cmd_set(), CMDSET_DDS);
        assert_eq!(packet3.cmd_id(), 0x01);

        assert_eq!(init4_view.num_topics(), 2);
        assert_eq!(init4_view.frequency(), 50);
        assert_eq!(init4_view.sub_node_id(), 0x09);
        assert_eq!(init4_view.sub_mode(), 0x00);
        assert_eq!(init4_view.sub_stream_id(), 0x01);

        let topics = init4_view.topics().unwrap();
        assert_eq!(
            topics[0].uid,
            [0xa7, 0x02, 0x29, 0x88, 0x03, 0x00, 0x02, 0x00] // Turret Yaw
        );
        assert_eq!(
            topics[1].uid,
            [0x66, 0x3e, 0x3e, 0x4c, 0x03, 0x00, 0x02, 0x00] // Velocity
        );
    }

    #[test]
    fn test_robostacks1_init_lab_odom() {
        let buf = [
            0x55, 0x3C, 0x04, 0xFF, 0x09, 0x03, 0x0F, 0x00, 0x40, 0x48, 0x03, 0x09, 0x02, 0x03,
            0x00, 0x05, 0x09, 0xA3, 0x26, 0xE2, 0x03, 0x00, 0x02, 0x00, 0xB3, 0xF7, 0xE6, 0x47,
            0x03, 0x00, 0x02, 0x00, 0xF4, 0x1D, 0x1C, 0xDC, 0x03, 0x00, 0x02, 0x00, 0x03, 0xC5,
            0x58, 0x08, 0x03, 0x00, 0x02, 0x00, 0x42, 0xEE, 0x13, 0x1D, 0x03, 0x00, 0x02, 0x00,
            0x05, 0x00, 0xFF, 0xFF,
        ];

        let packet = RMWireFrameView::new(&buf);
        let init_view = RMAddSubView::new(packet);

        assert_eq!(init_view.packet.cmd_set(), CMDSET_DDS);
        assert_eq!(init_view.packet.cmd_id(), CMDID_DDS_ADD_SUB);

        assert_eq!(init_view.num_topics(), 5);
        assert_eq!(init_view.frequency(), 5);
        assert_eq!(init_view.sub_node_id(), 0x09);
        assert_eq!(init_view.sub_mode(), 0x00);
        assert_eq!(init_view.sub_stream_id(), 0x02);

        let topics = init_view.topics().unwrap();
        assert_eq!(
            topics[0].uid,
            crate::vbus::topics::DDS_ESC_STATE // Wheel encoders
        );
        assert_eq!(
            topics[1].uid,
            crate::vbus::topics::DDS_BASE_POSITION // Bast Position
        );
        assert_eq!(
            topics[2].uid,
            crate::vbus::topics::DDS_IMU_DATA // IMU Data
        );
        assert_eq!(
            topics[3].uid,
            [0x03, 0xC5, 0x58, 0x08, 0x03, 0x00, 0x02, 0x00] // Odom Yaw
        );
        assert_eq!(
            topics[4].uid,
            [0x42, 0xEE, 0x13, 0x1D, 0x03, 0x00, 0x02, 0x00] // Odom Pitch
        );
    }

    #[test]
    fn test_heartbeat_packet() {
        let buf = [
            0x55, 0x1B, 0x04, 0xFF, 0x09, 0xC3, 0x00, 0x00, 0x00, 0x3F, 0x60, 0x00, 0x04, 0x20,
            0x00, 0x01, 0x00, 0x40, 0x00, 0x02, 0x10, 0x00, 0x03, 0x00, 0x00, 0xFF, 0xFF,
        ];

        let packet = RMWireFrameView::new(&buf);

        assert_eq!(packet.cmd_set(), 0x3F);
        assert_eq!(packet.cmd_id(), 0x60);
    }
}
