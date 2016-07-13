extern crate clap;
extern crate neoqo;
use std::fs::File;
use std::io::Read;
use clap::{Arg, App};
use neoqo::{Lexer, Optimizer, OptimizerPass, VirtualMachine};

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
        .arg(Arg::with_name("interpret")
            .short("i")
            .long("interpret")
            .takes_value(true))
        .arg(Arg::with_name("debug")
            .long("debug")
            .takes_value(false))
        .get_matches();

    // Retrieve the input code from the specified source
    let mut source = String::new();
    match matches.value_of("input") {

        // A filename was specified
        Some(filename) => {
            // Open the file
            let mut f = File::open(filename)
                .expect(&format!("Unable to open specified file: {}", filename));

            // Read the file
            f.read_to_string(&mut source)
                .expect(&format!("Unable to read the specified file: {}", filename));
        }

        // No filename was specified
        _ => {}
    }

    // Tokenize the source
    let mut lexer = Lexer::new(source);
    lexer.tokenize();

    // Run basic optimization passes
    //
    // A quick overview of why I choose this specific order
    // of optimization passes:
    //
    // 1) OptimizeIncDecPtrChains
    //    This pass cleans up pointer movement operations.
    //    By having this execute before the OptimizeIncDecValChains pass,
    //    the chance of eliminating effectively useless loops is higher,
    //    which makes the OptimizeIncDecValChains pass work better.
    // 2) OptimizeIncDecValChains
    //    This pass collapses multiple inc/dec cell value instructions
    //    into just one, which frees the way for the OptimizeClearLoops pass.
    // 3) OptimizeClearLoops
    //    This pass turns clear loops into a single clear instruction.
    //    It depends on a specific sequence of operations, which is why
    //    the OptimizeIncDecValChains pass should always run before this one.
    let mut optimizer = Optimizer::new(lexer.tokens.clone(), None);
    optimizer.add_pass(OptimizerPass::OptimizeIncDecPtrChains);
    optimizer.add_pass(OptimizerPass::OptimizeIncDecValChains);
    optimizer.add_pass(OptimizerPass::OptimizeClearLoops);
    optimizer.optimize(2);

    // Interpret the instructions
    let mut vm = VirtualMachine::new(optimizer.instructions, Option::None, Option::None);
    match matches.occurrences_of("debug") {
        0 => vm.run(),
        _ => vm.run_with_debugger(),
    }
}
