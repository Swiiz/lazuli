use std::{
    fs::{self, File},
    io::Read,
    path::PathBuf,
};

use clap::Parser;
use parser::parse;

mod parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    source_path: PathBuf,
}

fn main() {
    let args = Args::parse();
    let source = fs::read_to_string(args.source_path).expect("Invalid source path!");
    let tokens = parse(&source).expect("Invalid source: An error occured while parsing!");
    println!("{tokens:?}")
}
