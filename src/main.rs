extern crate clap;
use clap::{Arg, App};
use std::fs::File;
use std::io::Read;
mod lexer;
mod vm;
use lexer::Lexer;

fn main() {

    // Process command-line arguments
    let matches = App::new("neoqo")
        .version("0.0.1")
        .author("Splitty <splittdev@gmail.com>")
        .arg(Arg::with_name("input")
            .help("The input file.")
            .required(true)
            .index(1))
        .get_matches();

    // Get the filename
    let filename: &str = matches.value_of("input")
        .expect("Please specify an input file!");

    // Get the source out of the file
    let mut source = String::new();
    {
        // Open the file
        let mut f = File::open(filename)
            .expect(&format!("Unable to open specified file: {}", filename));

        // Read the file
        f.read_to_string(&mut source)
            .expect(&format!("Unable to read the specified file: {}", filename));
    }

    // Create the Lexer
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize();
}
