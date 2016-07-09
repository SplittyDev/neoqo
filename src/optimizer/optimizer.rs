use vm::instruction::Instruction;
use vm::opcodes::Opcode;
use optimizer::OPTIMIZED_VALUE;

/// The `OptimizerPass` type.
#[derive(Copy, Clone)]
pub enum OptimizerPass {
    /// Clear-loop optimization pass.
    ///
    /// Creates a single `Opcode::Clear` from `[-]`-style loops.
    OptimizeClearLoops,
}

/// The `Optimizer` type.
pub struct Optimizer {
    /// The optimization passes to be ran.
    passes: Vec<OptimizerPass>,

    /// The input instruction vector.
    pub in_instructions: Vec<Instruction>,

    /// The optimized instruction vector.
    out_instructions: Vec<Instruction>,
}

/// The `OptimizerPassState` type.
struct OptimizerPassState {
    /// The position in the `Optimizer.in_instructions` vector.
    pos: usize,

    /// The length of the `Optimizer.in_instructions` vector.
    size: usize,
}

/// The `Optimizer` implementation.
impl Optimizer {
    /// Constructs a new `Optimizer`.
    pub fn new(instructions: Vec<Instruction>, passes: Option<Vec<OptimizerPass>>) -> Optimizer {

        // Create the optimizer
        Optimizer {
            passes: passes.unwrap_or(Vec::new()),
            in_instructions: instructions.clone(),
            out_instructions: Vec::with_capacity(instructions.len()),
        }
    }

    /// Adds an optimization pass.
    pub fn add_pass(&mut self, pass: OptimizerPass) {
        self.passes.push(pass);
    }

    /// Runs the specified optimizations.
    pub fn optimize(&mut self) {
        let passes = self.passes.clone();
        for pass in passes {
            match pass {
                OptimizerPass::OptimizeClearLoops => self.optimize_clear_loops(),
            }
            self.in_instructions = self.out_instructions.clone();
            self.out_instructions = Vec::with_capacity(self.in_instructions.len());
        }
    }

    fn optimize_clear_loops(&mut self) {
        let mut state = self.create_pass_state();
        while state.can_advance(1) {
            let three = state.peek(self, 3);
            match three {
                Some(instr) => {
                    if (instr[0].is(Opcode::JzStack) || instr[0].is(Opcode::JzCell)) &&
                       instr[1].is(Opcode::Dec) &&
                       (instr[2].is(Opcode::JnzStack) || instr[2].is(Opcode::JnzCell)) {
                        self.out_instructions
                            .push(Instruction {
                                value: OPTIMIZED_VALUE.to_string(),
                                opcode: Opcode::Clear,
                                ..instr[0].clone()
                            });
                        state.skip(3);
                        continue;
                    }
                }
                _ => (),
            }
            self.out_instructions.push(self.in_instructions[state.pos].clone());
            state.skip(1);
        }
    }

    fn create_pass_state(&self) -> OptimizerPassState {
        OptimizerPassState {
            pos: 0,
            size: self.in_instructions.len(),
        }
    }
}

/// The `OptimizerPassState` implementation.
impl OptimizerPassState {
    /// Tests if reading `n` more characters is possible.
    fn can_advance(&self, n: usize) -> bool {
        self.pos + n < self.size
    }

    /// Peeks at the next `n` instructions.
    fn peek(&self, optimizer: &mut Optimizer, n: usize) -> Option<Vec<Instruction>> {
        match self.can_advance(n) {
            true => {
                let mut vec: Vec<Instruction> = Vec::with_capacity(n);
                vec.extend_from_slice(&optimizer.in_instructions[self.pos..(self.pos + n)]);
                Some(vec)
            }
            false => None,
        }
    }

    /// Skips the next `n` instructions.
    fn skip(&mut self, n: usize) {
        self.pos += n;
    }
}
