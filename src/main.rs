use std::{
    fs::{self, File},
    io::Read,
    path::PathBuf,
};

use clap::Parser;
use parser::parse;

use crate::ast::build_ast;

mod ast;
mod parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    source_path: PathBuf,
}

fn main() {
    let args = Args::parse();
    let source = fs::read_to_string(args.source_path).expect("Invalid source path!");
    let tokens = parse(&source).expect("An error occured while parsing!");
    println!("{tokens:?}");
    let ast = build_ast(tokens).expect("An error occured while building the abstract syntax tree!");
}
