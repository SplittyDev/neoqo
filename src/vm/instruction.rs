use vm::opcodes::Opcode;

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

    pub fn get_line_and_position(&self) -> (i32, i32) {
        (self.line, self.pos)
    }
}
