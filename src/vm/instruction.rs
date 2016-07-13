use std::fmt::{Display, Formatter, Result};
use vm::opcodes::Opcode;

/// The `Instruction` type.
#[derive(Clone, RustcEncodable, RustcDecodable, PartialEq, Debug)]
pub struct Instruction {
    /// The position relative to the line.
    pub pos: u32,

    /// The line.
    pub line: u32,

    /// The value of the instruction.
    pub value: String,

    /// The opcode.
    pub opcode: Opcode,

    /// The (optional) argument.
    pub argument: Option<u32>,

    /// A value indicating whether the instruction
    /// was modified by one or more optimization passes.
    pub optimized: bool,
}

/// The `Instruction` implementation.
impl Instruction {
    /// Constructs a new `Instruction.`
    pub fn new(pos: u32,
               line: u32,
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
            optimized: false,
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
