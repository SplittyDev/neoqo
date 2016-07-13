extern crate neoqo;
use neoqo::{DebugClient, DebugInformation};

// The main entry point of the application
fn main() {
    let mut client = DebugClient::new();
    print!("Connecting to debug server...");
    match client.connect() {
        true => println!("Connected!"),
        false => {
            println!("Failed!");
            println!("Please start neoqo first:\n $ neoqo <input> --debug");
            return;
        }
    }
    loop {
        match client.get_update() {
            None => continue,
            Some(data) => {
                if data.terminate {
                    println!("Debugger session terminated by server.");
                    return;
                }
                handle(data);
            }
        }
    }
}

fn handle(data: DebugInformation) {
    match data.instr {
        Some(instr) => {
            println!("[Line {:03}:{:03}] Opcode={:?}; Arg={:?}; Optimized={}",
                     instr.line,
                     instr.pos,
                     instr.opcode,
                     instr.argument,
                     instr.optimized);
        }
        None => (),
    }
}
