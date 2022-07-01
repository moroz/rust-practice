// #![allow(unused)]

use anyhow::{Context, Result};
use clap::Parser;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub mod error;

use crate::error::CustomError;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    // println!("{} {}", args.pattern, args.path.as_path().display());

    let f = File::open(&args.path).with_context(|| {
        format!(
            "could not open file `{}` for reading",
            &args.path.as_path().display()
        )
    })?;
    let mut reader = BufReader::new(f);
    let mut line = String::new();

    loop {
        line.clear();
        let bytes = reader.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }
        if line.contains(&args.pattern) {
            print!("{}", line);
        }
    }

    Ok(())
}
