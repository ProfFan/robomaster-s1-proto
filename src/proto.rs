//! RM-S1 CAN Bus Protocol
//!
//! The RM-S1 CAN bus is used as a stream bus. Each can frame is just another part of
//! the stream from one CAN ID.

use crate::crc::{rm_s1_crc16, rm_s1_crc8};

const SOF: u8 = 0x55;
const BASE_ID: u8 = 0x09;
const SUBCONTROLLER_ID: u8 = 0x03;
const CMDSET_LOG: u8 = 0xf0;

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    /// Not enough data to parse the frame
    ///
    /// The first usize is the number of bytes needed to parse the frame
    /// The second usize is the number of bytes consumed
    NeedMoreData(usize, usize),
    NoStartOfFrame,
    InvalidHeaderCRC(usize),
    InvalidPacketCRC(usize),
}

pub fn parse_frame(buffer: &[u8]) -> Result<(&[u8], usize), ParseError> {
    let mut idx = 0;

    // Find the start of frame
    while idx < buffer.len() {
        if buffer[idx] == SOF {
            break;
        }
        idx += 1;
    }

    if idx == buffer.len() {
        return Err(ParseError::NoStartOfFrame);
    }

    // Check if we have enough data to read the frame header
    if idx + 3 >= buffer.len() {
        return Err(ParseError::NeedMoreData(3, idx));
    }

    // Read the frame length
    let frame_len = buffer[idx + 1] as usize | ((buffer[idx + 2] & 0x03) as usize) << 8;

    // Check the header CRC
    let header_crc = buffer[idx + 3];
    let header = &buffer[idx..idx + 3];
    if rm_s1_crc8(header) != header_crc {
        return Err(ParseError::InvalidHeaderCRC(idx + 3));
    }

    // Check if we have enough data to read the whole frame
    if idx + frame_len > buffer.len() {
        return Err(ParseError::NeedMoreData(
            frame_len - (buffer.len() - idx),
            idx,
        ));
    }

    // Check the packet CRC
    let packet_header_body = &buffer[idx..idx + frame_len - 2];
    let packet_crc = &buffer[idx + frame_len - 2..idx + frame_len];
    if rm_s1_crc16(packet_header_body) != u16::from_le_bytes([packet_crc[0], packet_crc[1]]) {
        // Skip the header so we can find the next frame
        return Err(ParseError::InvalidPacketCRC(idx + 3));
    }

    Ok((&buffer[idx..idx + frame_len], idx + frame_len))
}

#[cfg(test)]
mod tests {
    extern crate std;
    use super::*;

    #[test]
    fn test_no_sof() {
        let buffer = [0x00, 0x00, 0x0E, 0x04, 0x66, 0x09, 0x03, 0x4E, 0x06];
        let result = parse_frame(&buffer);
        assert_eq!(result, Err(ParseError::NoStartOfFrame));
    }

    #[test]
    fn test_header_not_enough_data() {
        let buffer = [0x00, 0x55, 0x0E, 0x04];
        let result = parse_frame(&buffer);
        assert_eq!(result, Err(ParseError::NeedMoreData(3, 1)));
    }

    #[test]
    fn test_header_invalid_crc() {
        let buffer = [0x00, 0x55, 0x0E, 0x00, 0x66, 0x09, 0x03, 0x4E, 0x06];
        let result = parse_frame(&buffer);
        assert_eq!(result, Err(ParseError::InvalidHeaderCRC(4)));
    }

    #[test]
    fn test_need_more_data() {
        let buffer = [0x00, 0x55, 0x0E, 0x04, 0x66, 0x09, 0x03, 0x4E, 0x06];
        let result = parse_frame(&buffer);
        assert_eq!(result, Err(ParseError::NeedMoreData(6, 1)));
    }

    #[test]
    fn test_full_parse() {
        let input = [
            0x00, 0x55, 0x0E, 0x04, 0x66, 0x09, 0x03, 0x4E, 0x06, 0xA0, 0x48, 0x08, 0x01, 0xC2,
            0xE8,
        ];

        let result = parse_frame(&input);

        assert_eq!(result, Ok((&input[1..], input.len())));
    }

    #[test]
    fn test_stream_state_machine() {
        let mut buffer = [0u8; 64];
        let input1 = [0x00, 0x55, 0x0E, 0x04, 0x66, 0x09, 0x03, 0x4E, 0x06];
        let input2 = [0xA0, 0x48, 0x08, 0x01, 0xC2, 0xE8];

        let result = parse_frame(&input1);
        assert_eq!(result, Err(ParseError::NeedMoreData(6, 1)));

        buffer[..input1.len()].copy_from_slice(&input1);
        buffer[input1.len()..input1.len() + 6].copy_from_slice(&input2);

        let result = parse_frame(&buffer);
        assert_eq!(result, Ok((&buffer[1..0xF], 0xF)));
    }
}
