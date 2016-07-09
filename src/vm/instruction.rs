use std::fmt::{Display, Formatter, Result};
use vm::opcodes::Opcode;

/// The `Instruction` type.
#[derive(Clone)]
pub struct Instruction {
    /// The position relative to the line.
    pos: i32,

    /// The line.
    line: i32,

    /// The value of the instruction.
    pub value: String,

    /// The opcode.
    pub opcode: Opcode,
}

/// The `Instruction` implementation.
impl Instruction {
    /// Constructs a new `Instruction.`
    pub fn new(pos: i32, line: i32, value: String, opcode: Opcode) -> Instruction {

        // Create the instruction
        Instruction {
            pos: pos,
            line: line,
            value: value,
            opcode: opcode,
        }
    }
}

/// Implements the `Display` trait for the `Instruction` type.
impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "({}:{})", self.line, self.pos)
    }
}
