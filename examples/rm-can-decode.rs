use std::{collections::HashMap, io::BufRead, path::PathBuf};

use candump_parse;
use chumsky::Parser;
use robomaster_s1_proto::{self, duss::cmd_set_types::CommandSetType};

use clap::Parser as ClapParser;

/// Simple program to greet a person
#[derive(ClapParser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    input: PathBuf,
}

fn show_buf<B: AsRef<[u8]>>(buf: B) -> String {
    String::from_utf8(
        buf.as_ref()
            .iter()
            .map(|b| core::ascii::escape_default(*b))
            .flatten()
            .collect(),
    )
    .unwrap()
}

fn print_packet(id: u32, packet: &[u8]) {
    let view = robomaster_s1_proto::wire::RMWireFrameView::new(packet);
    if view.is_valid() {
        if view.cmd_set() == robomaster_s1_proto::duss::vbus::CMDSET_VBUS {
            match view.cmd_id() {
                robomaster_s1_proto::duss::vbus::CMDID_VBUS_ADD_SUB => {
                    let topic_view =
                        robomaster_s1_proto::duss::vbus::topic_view::RMAddSubView::new(view);
                    println!(
                        "{:#0x}: VBUS Add Sub: STR {}, {}{}, {:02x?}",
                        id,
                        topic_view.sub_stream_id(),
                        if topic_view.packet.need_ack() {
                            "A"
                        } else {
                            "_"
                        },
                        if topic_view.packet.is_ack() { "K" } else { "_" },
                        topic_view.topics()
                    );
                }
                robomaster_s1_proto::duss::vbus::CMDID_VBUS_DEL_SUB => {
                    println!("{:#0x}: VBUS Del Sub: {:02x?}", id, view.payload());
                }
                robomaster_s1_proto::duss::vbus::CMDID_VBUS_RESET_NODE => {
                    println!("{:#0x}: VBUS Reset Node {}", id, view.receiver_id());
                }
                robomaster_s1_proto::duss::vbus::CMDID_VBUS_PUSH_MSG => {
                    let topic_view = robomaster_s1_proto::duss::vbus::RMTopicView::new(view);
                    if topic_view.sub_mode() == 0 {
                        println!(
                            "{:#0x}: VBUS PUSH Stream: {}, {}{}, DATA {:02x?}",
                            id,
                            topic_view.sub_id(),
                            if topic_view.packet.need_ack() {
                                "A"
                            } else {
                                "_"
                            },
                            if topic_view.packet.is_ack() { "K" } else { "_" },
                            topic_view.data()
                        );
                    } else {
                        println!(
                            "{:#0x}: VBUS PUSH ACK, {}{}, PAYLOAD {:02x?}",
                            id,
                            if topic_view.packet.need_ack() {
                                "A"
                            } else {
                                "_"
                            },
                            if topic_view.packet.is_ack() { "K" } else { "_" },
                            topic_view.packet.payload()
                        );
                    }
                }
                _ => {
                    println!(
                        "{:#0x}: {:02x} to {:02x}, CS {:02x}, CMD {:02x}, {}",
                        id,
                        view.sender_id(),
                        view.receiver_id(),
                        view.cmd_set(),
                        view.cmd_id(),
                        show_buf(view.payload())
                    );
                }
            }
        } else {
            println!(
                "{:#0x}: {:02x} to {:02x}, {}{}, CS {:?}, CMD {:02x}, {}",
                id,
                view.sender_id(),
                view.receiver_id(),
                if view.need_ack() { "A" } else { "_" },
                if view.is_ack() { "K" } else { "_" },
                CommandSetType::try_from(view.cmd_set()),
                view.cmd_id(),
                show_buf(view.payload())
            );
        }
    } else {
        println!("Invalid packet");
    }
}

fn main() {
    let args = Args::parse();

    // Open a file reader (line-by-line)
    let file = std::fs::File::open(&args.input).unwrap();

    let mut reader = std::io::BufReader::new(file);

    // Each CAN node id has a buffer
    let mut buffers: HashMap<u32, Vec<u8>> = HashMap::new();

    // Parse each line
    let mut line = String::new();
    while reader.read_line(&mut line).unwrap() > 0 {
        let result = candump_parse::parser().parse(line.as_str());
        match result {
            Ok(frame) => {
                let id = frame.id;
                let buf = buffers.entry(id).or_default();

                // Append the data to the buffer
                buf.extend_from_slice(&frame.data);

                // Try to parse the RM-S1 frame
                let result = robomaster_s1_proto::proto::parse_frame(buf.as_slice());

                match result {
                    Ok((packet, consumed)) => {
                        print_packet(id, &packet);
                        buf.drain(0..consumed);
                    }
                    Err(robomaster_s1_proto::proto::ParseError::NeedMoreData(_, _)) => {
                        // eprintln!("Need more data: {}", needed);
                    }
                    Err(robomaster_s1_proto::proto::ParseError::NoStartOfFrame) => {
                        // eprintln!("No start of frame");
                        // Drain the buffer
                        buf.clear();
                    }
                    Err(robomaster_s1_proto::proto::ParseError::InvalidHeaderCRC(consumed)) => {
                        eprintln!("Invalid header CRC, skipping {}", consumed);
                        buf.drain(0..consumed);
                    }
                    Err(robomaster_s1_proto::proto::ParseError::InvalidPacketCRC(consumed)) => {
                        eprintln!("{:#02x}: Invalid payload CRC, skipping {}", id, consumed);
                        buf.drain(0..consumed);
                    }
                }
            }
            Err(e) => {
                eprintln!("Error: {:?}", e);
            }
        }
        line.clear();
    }
}
