use std::collections::BTreeMap;
use std::cmp::max;
use std::char;
use vm::instruction::Instruction;
use vm::opcodes::Opcode;

/// The `PrintMode` type.
enum PrintMode {
    /// The `.` instruction prints characters
    Char,

    /// The `.` instruction prints integers
    Integer,
}

/// The `VirtualMachine` type.
pub struct VirtualMachine {
    /// The instructions to be processed
    instructions: Vec<Instruction>,

    /// The instruction pointer
    ip: usize,

    /// The cell pointer
    cp: usize,

    /// The ticks; more specifically the the cycle count
    ticks: u64,

    /// The memory; each item represents a cell.
    memory: Vec<u32>,

    /// The stack.
    stack: Vec<u32>,

    /// The jump table; used for correctly handling nested loops.
    jump_table: BTreeMap<usize, usize>,

    /// The print mode. See `PrintMode` for details.
    print_mode: PrintMode,
}

/// The `VirtualMachine` implementation.
impl VirtualMachine {
    /// Constructs a new `VirtualMachine`.
    pub fn new(instructions: Vec<Instruction>,
               memory_size: Option<usize>,
               stack_size: Option<usize>)
               -> VirtualMachine {

        // Default values
        const DEFAULT_STACK_SIZE: usize = 64;
        const DEFAULT_MEMORY_SIZE: usize = 128;

        // Create the virtual machine
        VirtualMachine {
            ip: 0usize,
            cp: 0usize,
            ticks: 0u64,
            instructions: instructions,
            jump_table: BTreeMap::new(),
            print_mode: PrintMode::Char,
            stack: Vec::with_capacity(stack_size.unwrap_or(DEFAULT_STACK_SIZE)),
            memory: vec![0; memory_size.unwrap_or(DEFAULT_MEMORY_SIZE)],
        }
    }

    /// Interprets the loaded instructions.
    pub fn run(&mut self) {

        // Build the jump table
        // This is important for nested loops to work properly
        self.build_jump_table();

        // Keep running code until the instruction pointer
        // is equal to or bigger than the instruction count
        while self.ip < self.instructions.len() {

            // Execute the next instruction
            self.run_cycle();
        }
    }

    /// Builds the jump table.
    fn build_jump_table(&mut self) {

        // Create a loop stack which stores the positions
        // of the cell and stack loop instructions
        let mut loop_stack: Vec<usize> = Vec::with_capacity(8);
        let mut instr_stack: Vec<Instruction> = Vec::new();

        // Iterate over the instructions
        for i in 0..self.instructions.len() {

            // Match the current opcode against the conditional jump operators
            match self.instructions[i].opcode {

                // Jump if zero
                Opcode::JzCell | Opcode::JzStack => {
                    loop_stack.push(i);
                    instr_stack.push(self.instructions[i].clone());
                }

                // Jump if not zero
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

                // Ignore the rest
                _ => (),
            }
        }

        // Test if the loop stack still contains any items
        if loop_stack.len() > 0usize {

            // If so, panic!
            let instr = instr_stack.pop().unwrap();
            panic!(format!("Unmatched '{}' at position {}", instr.value, instr));
        }
    }

    /// Executes a single instruction.
    fn run_cycle(&mut self) {
        let instr = &self.instructions[self.ip];
        match instr.opcode {

            // Increment the cell pointer
            Opcode::IncPtr => self.cp += 1,

            // Decrement the cell pointer
            Opcode::DecPtr => self.cp = max(0, self.cp - 1),

            // Increment the cell value
            Opcode::Inc => self.memory[self.cp] += 1,

            // Decrement the cell value
            Opcode::Dec => self.memory[self.cp] = max(0, self.memory[self.cp] - 1),

            // Double the cell value
            Opcode::Double => self.memory[self.cp] *= 2,

            // Halve the cell value
            Opcode::Halve => self.memory[self.cp] /= 2,

            // Push the cell value onto the stack
            Opcode::Push => self.stack.push(self.memory[self.cp]),

            // Set the value of the cell to the number of items on the stack
            Opcode::Count => self.memory[self.cp] = self.stack.len() as u32,

            // Set the printing mode to `Char`, see `PrintMode` for details.
            Opcode::ChrMod => self.print_mode = PrintMode::Char,

            // Set the printing mode to `Integer`, see `PrintMode` for details.
            Opcode::IntMod => self.print_mode = PrintMode::Integer,

            // Print the value of the current cell
            // The output format depends on the printing mode, see `PrintMode` for details.
            Opcode::Print => {
                match self.print_mode {
                    PrintMode::Char => print!("{}", char::from_u32(self.memory[self.cp]).unwrap()),
                    PrintMode::Integer => print!("{}", self.memory[self.cp]),
                }
            }

            // Push a string onto the stack, followed by a zero value
            // The string is pushed in reverse order, that way it can be easily processed.
            Opcode::Str => {
                let vec: Vec<char> = instr.value.chars().collect();
                self.stack.push(0);
                for i in 0..vec.len() {
                    self.stack.push(vec[vec.len() - i - 1] as u32);
                }
            }

            // Jump to the end of the loop if the value of the cell is zero
            Opcode::JzCell => {
                if self.memory[self.cp] == 0 {
                    self.ip = self.jump_table[&self.ip];
                }
            }

            // Jump to the beginning of the loop if the value of the cell is not zero
            Opcode::JnzCell => {
                if self.memory[self.cp] != 0 {
                    self.ip = self.jump_table[&self.ip];
                }
            }

            // Pop the top value off the stack and assign it to the cell
            Opcode::Pop => self.memory[self.cp] = self.stack.pop().unwrap_or(0),

            // Duplicate the top value on the stack
            Opcode::Dup => {
                let stack_length = self.stack.len();
                if stack_length == 0 {
                    panic!(format!("Attempt to pop value off empty stack (at {})", instr));
                }
                let value = self.stack[stack_length - 1];
                self.stack.push(value);
            }

            // Swap the top two values on the stack with each other
            // Useful for keeping track of state while popping a sequence of items off the stack.
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

            // Compare the top two values on the stack
            // Assign 0 (false) or 1 (true) to the cell, depending on the result of the comparison.
            Opcode::Compare => {
                let stack_length = self.stack.len();
                if stack_length < 2 {
                    panic!(format!("Attempt to pop value off empty stack (at {})", instr));
                }
                let fst = self.stack[stack_length - 1];
                let snd = self.stack[stack_length - 2];
                self.memory[self.cp] = (fst == snd) as u32;
            }

            // Jump to the end of the loop if the top value on the stack is zero
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

            // Jump to the beginning of the loop if the top value on the stack is not zero
            Opcode::JnzStack => {
                let stack_len = self.stack.len();
                if stack_len > 0 && self.stack[stack_len - 1] != 0 {
                    self.ip = self.jump_table[&self.ip];
                }
            }

            // Panic if an unknown instruction is encountered
            _ => panic!(format!("Unimplemented instruction '{}' (at {})", instr.value, instr)),
        }

        // Increment the instruction pointer and the tick value
        self.ip += 1;
        self.ticks += 1;
    }
}
