use anyhow::Result;
use std::env;
use std::fs::File;
use std::io::{self, Read, Write};
use std::process;

mod lz;
mod rle;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        eprintln!(
            "Usage: {} [compress|decompress] [--rle|--lz] [input_file|-] [output_file|-]",
            args[0]
        );
        process::exit(1);
    }

    let operation = &args[1];
    let algorithm = &args[2];
    let input_path = &args[3];
    let output_path = &args[4];

    // Determine input source
    let mut input: Box<dyn Read> = if input_path == "-" {
        Box::new(io::stdin())
    } else {
        Box::new(File::open(input_path)?)
    };

    // Determine output destination
    let mut output: Box<dyn Write> = if output_path == "-" {
        Box::new(io::stdout())
    } else {
        Box::new(File::create(output_path)?)
    };

    // Process data based on operation and algorithm
    match (operation.as_str(), algorithm.as_str()) {
        ("compress", "--rle") => rle::compress(&mut input, &mut output)?,
        ("decompress", "--rle") => rle::decompress(&mut input, &mut output)?,
        ("compress", "--lz") => lz::compress(&mut input, &mut output)?,
        ("decompress", "--lz") => lz::decompress(&mut input, &mut output)?,
        _ => {
            eprintln!("Invalid operation or algorithm");
            process::exit(1);
        }
    }

    output.flush()?;
    Ok(())
}
