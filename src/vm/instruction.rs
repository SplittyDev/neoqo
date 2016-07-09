use std::fmt::{Display, Formatter, Result};
use vm::opcodes::Opcode;

/// The `Instruction` type.
#[derive(Clone)]
pub struct Instruction {
    /// The position relative to the line.
    pub pos: i32,

    /// The line.
    pub line: i32,

    /// The value of the instruction.
    pub value: String,

    /// The opcode.
    pub opcode: Opcode,

    /// The (optional) argument.
    pub argument: Option<u32>,
}

/// The `Instruction` implementation.
impl Instruction {
    /// Constructs a new `Instruction.`
    pub fn new(pos: i32,
               line: i32,
               value: String,
               opcode: Opcode,
               arg: Option<u32>)
               -> Instruction {

        // Create the instruction
        Instruction {
            pos: pos,
            line: line,
            value: value,
            opcode: opcode,
            argument: arg,
        }
    }

    pub fn is(&self, opcode: Opcode) -> bool {
        self.opcode == opcode
    }
}

/// Implements the `Display` trait for the `Instruction` type.
impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "({}:{})", self.line, self.pos)
    }
}
