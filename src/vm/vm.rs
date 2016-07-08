use std::collections::BTreeMap;
use vm::instruction::Instruction;
use vm::opcodes::Opcode;

#[allow(dead_code)]
pub struct VirtualMachine {
    instructions: Vec<Instruction>,
    ip: usize,
    cp: usize,
    ticks: u64,
    memory: Vec<i32>,
    stack: Vec<i32>,
    jump_table: BTreeMap<usize, usize>,
}

impl VirtualMachine {
    pub fn new(instructions: Vec<Instruction>,
               memory_size: Option<usize>,
               stack_size: Option<usize>)
               -> VirtualMachine {
        VirtualMachine {
            ip: 0usize,
            cp: 0usize,
            ticks: 0u64,
            instructions: instructions,
            jump_table: BTreeMap::new(),
            stack: vec![0; stack_size.unwrap_or(64)],
            memory: vec![0; memory_size.unwrap_or(1024)],
        }
    }

    pub fn run(&mut self) {
        self.build_jump_table();
    }

    fn build_jump_table(&mut self) {
        let mut loop_stack = vec![0usize];
        let mut instr_stack: Vec<Instruction> = Vec::new();
        for i in 0..self.instructions.len() {
            match self.instructions[i].opcode {
                Opcode::JnzCell | Opcode::JnzStack => {
                    loop_stack.push(i);
                    instr_stack.push(self.instructions[i].clone());
                }
                Opcode::JzCell | Opcode::JzStack => {
                    let instr = self.instructions[i].clone();
                    if loop_stack.len() == 0usize {
                        panic!(format!("Unmatched '{}' at position {}", instr.value, instr));
                    }
                    instr_stack.pop();
                    let start = loop_stack.pop().unwrap();
                    self.jump_table.insert(start, i);
                    self.jump_table.insert(i, start);
                }
                _ => (),
            }
        }
        if loop_stack.len() > 0usize {
            let instr = instr_stack.pop().unwrap();
            panic!(format!("Unmatched '{}' at position {}", instr.value, instr));
        }
    }

    #[allow(dead_code)]
    fn run_cycle(&mut self) {
        let instr = &self.instructions[self.ip];
        match instr.opcode {
            _ => panic!(format!("Unimplemented opcode: {}", self.instructions[self.ip].value)),
        }
    }
}
