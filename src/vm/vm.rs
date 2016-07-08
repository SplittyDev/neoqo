use vm::instruction::Instruction;

pub struct VirtualMachine {
    instructions: Vec<Instruction>,
    ip: usize,
    ticks: u64,
    memory: Vec<i32>,
    stack: Vec<i32>,
}

impl VirtualMachine {
    pub fn new(instructions: Vec<Instruction>,
               memory_size: Option<usize>,
               stack_size: Option<usize>)
               -> VirtualMachine {
        VirtualMachine {
            ip: 0usize,
            ticks: 0u64,
            instructions: instructions,
            stack: vec![0; memory_size.unwrap_or(64)],
            memory: vec![0; memory_size.unwrap_or(1024)],
        }
    }

    pub fn run(&mut self) {
        unimplemented!();
    }

    fn run_cycle(&mut self) {
        unimplemented!();
    }
}
