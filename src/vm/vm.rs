use std::collections::BTreeMap;
use std::cmp::max;
use std::char;
use vm::instruction::Instruction;
use vm::opcodes::Opcode;

enum PrintMode {
    Char,
    Integer,
}

pub struct VirtualMachine {
    instructions: Vec<Instruction>,
    ip: usize,
    cp: usize,
    ticks: u64,
    memory: Vec<u32>,
    stack: Vec<u32>,
    jump_table: BTreeMap<usize, usize>,
    print_mode: PrintMode,
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
            print_mode: PrintMode::Char,
            stack: vec![0; stack_size.unwrap_or(64)],
            memory: vec![0; memory_size.unwrap_or(1024)],
        }
    }

    pub fn run(&mut self) {
        self.build_jump_table();
        while self.ip < self.instructions.len() {
            self.run_cycle();
        }
    }

    fn build_jump_table(&mut self) {
        let mut loop_stack: Vec<usize> = Vec::with_capacity(8);
        let mut instr_stack: Vec<Instruction> = Vec::new();
        for i in 0..self.instructions.len() {
            match self.instructions[i].opcode {
                Opcode::JzCell | Opcode::JzStack => {
                    loop_stack.push(i);
                    instr_stack.push(self.instructions[i].clone());
                }
                Opcode::JnzCell | Opcode::JnzStack => {
                    let instr = self.instructions[i].clone();
                    if loop_stack.len() == 0 {
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

    fn run_cycle(&mut self) {
        let instr = &self.instructions[self.ip];
        match instr.opcode {
            Opcode::IncPtr => self.cp += 1,
            Opcode::DecPtr => self.cp = max(0, self.cp - 1),
            Opcode::Inc => self.memory[self.cp] += 1,
            Opcode::Dec => self.memory[self.cp] = max(0, self.memory[self.cp - 1]),
            Opcode::Double => self.memory[self.cp] *= 2,
            Opcode::Halve => self.memory[self.cp] /= 2,
            Opcode::Push => self.stack.push(self.memory[self.cp]),
            Opcode::Count => self.memory[self.cp] = self.stack.len() as u32,
            Opcode::ChrMod => self.print_mode = PrintMode::Char,
            Opcode::IntMod => self.print_mode = PrintMode::Integer,
            Opcode::Print => {
                match self.print_mode {
                    PrintMode::Char => print!("{}", char::from_u32(self.memory[self.cp]).unwrap()),
                    PrintMode::Integer => print!("{}", self.memory[self.cp]),
                }
            }
            Opcode::Str => {
                let vec: Vec<char> = instr.value.chars().collect();
                for i in 0..vec.len() {
                    self.stack.push(vec[vec.len() - i - 1] as u32);
                }
            }
            Opcode::JzCell => {
                if self.memory[self.cp] == 0 {
                    self.ip = self.jump_table[&self.ip];
                }
            }
            Opcode::JnzCell => {
                if self.memory[self.cp] != 0 {
                    self.ip = self.jump_table[&self.ip];
                }
            }
            Opcode::Pop => self.memory[self.cp] = self.stack.pop().unwrap_or(0),
            Opcode::Dup => {
                let stack_length = self.stack.len();
                if stack_length == 0 {
                    panic!(format!("Attempt to pop value off empty stack (at {})", instr));
                }
                let value = self.stack[stack_length - 1];
                self.stack.push(value);
            }
            Opcode::Swap => {
                let stack_length = self.stack.len();
                if stack_length < 2 {
                    panic!(format!("Attempt to pop value off empty stack (at {})", instr));
                }
                let fst = self.stack[stack_length - 1];
                let snd = self.stack[stack_length - 2];
                self.stack.push(fst);
                self.stack.push(snd);
            }
            Opcode::Compare => {
                let stack_length = self.stack.len();
                if stack_length < 2 {
                    panic!(format!("Attempt to pop value off empty stack (at {})", instr));
                }
                let fst = self.stack[stack_length - 1];
                let snd = self.stack[stack_length - 2];
                self.memory[self.cp] = (fst == snd) as u32;
            }
            Opcode::JzStack => {
                let stack_len = self.stack.len();
                match stack_len {
                    0 => self.ip = self.jump_table[&self.ip],
                    _ => {
                        if self.stack[stack_len - 1] == 0 {
                            self.ip = self.jump_table[&self.ip];
                        }
                    }
                }
            }
            Opcode::JnzStack => {
                let stack_len = self.stack.len();
                if stack_len > 0 && self.stack[stack_len - 1] != 0 {
                    self.ip = self.jump_table[&self.ip];
                }
            }
            _ => panic!(format!("Unimplemented instruction '{}' (at {})", instr.value, instr)),
        }
        self.ip += 1;
        self.ticks += 1;
    }
}
