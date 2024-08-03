use std::{io::BufRead, path::PathBuf};

use chumsky::prelude::*;
use robomaster_s1_proto;

use clap::Parser as ClapParser;

/// Simple program to greet a person
#[derive(ClapParser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    input: PathBuf,
}

/// The Linux `candump` file format frame.
///
/// # Example
///
/// Input:
/// ```
/// (0000000000.000000) vcan0 211#616C75653A353131
/// (0000000000.000000) vcan0 212#0D0A4104
/// ```
#[derive(Debug)]
struct CandumpFrame {
    timestamp: core::time::Duration,
    interface: String,
    id: u32,
    data: Vec<u8>,
}

fn parser() -> impl Parser<char, CandumpFrame, Error = Simple<char>> {
    let time_unix = text::digits(10).map(|s: String| s.parse().unwrap());
    let time_frac = text::digits(10).map(|s: String| s.parse().unwrap());

    let timestamp = time_unix
        .then_ignore(just("."))
        .then(time_frac)
        .map(|(unix, frac): (u64, u32)| core::time::Duration::new(unix, frac * 1000u32))
        .delimited_by(just('('), just(')'));

    let interface = text::ident();

    let can_id = text::digits(16).map(|s: String| u32::from_str_radix(&s, 16).unwrap());
    let can_data = text::digits(16).map(|s: String| {
        s.as_bytes()
            .chunks(2)
            .map(|chunk| u8::from_str_radix(std::str::from_utf8(chunk).unwrap(), 16).unwrap())
            .collect()
    });

    let frame = can_id.then_ignore(just("#")).then(can_data);

    let expr = timestamp
        .then_ignore(just(" "))
        .then(interface)
        .then_ignore(just(" "))
        .then(frame)
        .map(|((timestamp, interface), (id, data))| CandumpFrame {
            timestamp,
            interface,
            id,
            data,
        });

    expr
}

fn main() {
    let args = Args::parse();

    // Open a file reader (line-by-line)
    let file = std::fs::File::open(&args.input).unwrap();

    let mut reader = std::io::BufReader::new(file);

    // Parse each line
    let mut line = String::new();
    while reader.read_line(&mut line).unwrap() > 0 {
        let result = parser().parse(line.as_str());
        match result {
            Ok(frame) => {
                println!("{:0x?}", frame);
            }
            Err(e) => {
                eprintln!("Error: {:?}", e);
            }
        }
        line.clear();
    }
}
