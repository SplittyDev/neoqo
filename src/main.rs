extern crate clap;
use clap::{Arg, App};
use std::fs::File;
use std::io::Read;
mod lexer;
use lexer::Lexer;
mod optimizer;
use optimizer::{Optimizer, OptimizerPass};
mod vm;
use vm::VirtualMachine;

/// The main entry point of the application.
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

    // Tokenize the source
    let mut lexer = Lexer::new(source);
    lexer.tokenize();

    // Run basic optimization passes
    let mut optimizer = Optimizer::new(lexer.tokens.clone(), None);
    optimizer.add_pass(OptimizerPass::OptimizeClearLoops);
    optimizer.optimize();

    // Interpret the instructions
    let mut vm = VirtualMachine::new(optimizer.in_instructions, Option::None, Option::None);
    vm.run();
}
