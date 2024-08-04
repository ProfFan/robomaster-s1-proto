use std::{io::BufRead, path::PathBuf};

use candump_parse;
use chumsky::Parser;

use clap::Parser as ClapParser;

/// Simple program to greet a person
#[derive(ClapParser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    input: PathBuf,
}

fn main() {
    let args = Args::parse();

    // Open a file reader (line-by-line)
    let file = std::fs::File::open(&args.input).unwrap();

    let mut reader = std::io::BufReader::new(file);

    // Parse each line
    let mut line = String::new();
    while reader.read_line(&mut line).unwrap() > 0 {
        let result = candump_parse::parser().parse(line.as_str());
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
