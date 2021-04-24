use std::env;
use std::fs;
use std::io;
use std::process::{self, Command};

mod error;
mod expr;
mod generator;
mod kind;
mod parser;
mod scanner;
mod stmt;
mod token;
mod value;
mod variant;

use crate::generator::Generator;
use crate::parser::Parser;
use crate::scanner::Scanner;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 2 {
        println!("usage: blaze [script] [output]");
        process::exit(1);
    } else {
        let source = fs::read_to_string(&args[0])?;
        let destination = format!("{}.rs", &args[1]);

        let mut scanner = Scanner::new(&source);
        let (tokens, errors) = scanner.scan();

        for error in errors.iter() {
            eprintln!("{}", error);
        }

        if !errors.is_empty() {
            process::exit(1);
        }

        let mut parser = Parser::new(tokens);
        let (statements, errors) = parser.parse();

        for error in errors.iter() {
            eprintln!("{}", error);
        }

        if !errors.is_empty() {
            process::exit(1);
        }

        let mut generator = Generator::new();
        let (output, errors) = generator.generate(&statements);

        for error in errors.iter() {
            eprintln!("{}", error);
        }

        if !errors.is_empty() {
            process::exit(1);
        }

        fs::write(&destination, output)?;

        let status = Command::new("rustc")
            .arg("-O")
            .arg(&destination)
            .status()
            .expect("rustc is missing");

        process::exit(status.code().unwrap_or(0));
    }
}
