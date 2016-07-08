use std::fmt::{Display, Formatter, Result};
use vm::opcodes::Opcode;

#[derive(Clone)]
pub struct Instruction {
    pos: i32,
    line: i32,
    pub value: String,
    pub opcode: Opcode,
}

impl Instruction {
    pub fn new(pos: i32, line: i32, value: String, opcode: Opcode) -> Instruction {
        Instruction {
            pos: pos,
            line: line,
            value: value,
            opcode: opcode,
        }
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "({}:{})", self.line, self.pos)
    }
}
