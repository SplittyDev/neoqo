mod server;
mod client;
use debugger::server::Server;
use debugger::client::Client;
use vm::instruction::Instruction;

/// The `DebugServer` type.
pub type DebugServer = Server;

/// The `DebugClient` type.
pub type DebugClient = Client;

/// The port used by the debugger for communication.
pub const DEBUGGER_PORT: u16 = 38100;

/// The `DebugInformation` type.
#[derive(RustcEncodable, RustcDecodable, PartialEq, Debug)]
pub struct DebugInformation {
    /// The current instruction.
    pub instr: Instruction,
}
