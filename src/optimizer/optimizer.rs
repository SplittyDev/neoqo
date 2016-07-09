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

    /// Increment/decrement chain optimization pass.
    ///
    /// Creates a single `Opcode::Inc` or `Opcode::Dec` from a
    /// chain of cell increment and decrement operators.
    OptimizeIncDecValChains,

    /// Increment/decrement pointer chain optimization pass.
    ///
    /// Create a single `Opcode::IncPtr` or `Opcode::DecPtr` from a
    /// chain of cell pointer increment and decrement operators.
    OptimizeIncDecPtrChains,
}

/// The `Optimizer` type.
pub struct Optimizer {
    /// The optimization passes to be ran.
    passes: Vec<OptimizerPass>,

    /// The input instruction vector.
    pub instructions: Vec<Instruction>,

    /// The optimized instruction vector.
    out_instructions: Vec<Instruction>,
}

/// The `OptimizerPassState` type.
struct OptimizerPassState {
    /// The position in the `Optimizer.instructions` vector.
    pos: usize,

    /// The length of the `Optimizer.instructions` vector.
    size: usize,
}

/// The `Optimizer` implementation.
impl Optimizer {
    /// Constructs a new `Optimizer`.
    pub fn new(instructions: Vec<Instruction>, passes: Option<Vec<OptimizerPass>>) -> Optimizer {

        // Create the optimizer
        Optimizer {
            passes: passes.unwrap_or(Vec::new()),
            instructions: instructions.clone(),
            out_instructions: Vec::with_capacity(instructions.len()),
        }
    }

    /// Adds an optimization pass.
    pub fn add_pass(&mut self, pass: OptimizerPass) {
        self.passes.push(pass);
    }

    /// Runs the specified optimizations.
    pub fn optimize(&mut self, iterations: usize) {
        if self.passes.len() == 0 || iterations == 0 {
            return;
        }
        for _ in 0..iterations {
            for pass in self.passes.clone() {
                let mut state = self.create_pass_state();
                while state.can_advance(0) {
                    let changed = match pass {
                        OptimizerPass::OptimizeClearLoops => self.optimize_clear_loops(&mut state),
                        OptimizerPass::OptimizeIncDecValChains => {
                            self.optimize_inc_dec_val_chains(&mut state)
                        }
                        OptimizerPass::OptimizeIncDecPtrChains => {
                            self.optimize_inc_dec_ptr_chains(&mut state)
                        }
                    };
                    if !changed {
                        self.out_instructions.push(self.instructions[state.pos].clone());
                        state.skip(1);
                    }
                }
                self.instructions = self.out_instructions.clone();
                self.out_instructions.clear();
            }
        }
    }

    fn optimize_clear_loops(&mut self, state: &mut OptimizerPassState) -> bool {
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
                    return true;
                }
                false
            }
            _ => false,
        }
    }

    fn optimize_inc_dec_val_chains(&mut self, state: &mut OptimizerPassState) -> bool {
        let mut incs = 0;
        let mut decs = 0;
        let fst = state.peek_at(self, 0).unwrap();
        while state.can_advance(0) {
            let current = state.peek_at(self, 0);
            match current {
                Some(val) => {
                    match val.opcode {
                        Opcode::Inc => incs += val.argument.unwrap_or(1),
                        Opcode::Dec => decs += val.argument.unwrap_or(1),
                        _ => break,
                    }
                    state.skip(1);
                }
                None => break,
            }
        }
        if incs != 0 || decs != 0 {
            self.out_instructions.push(Instruction {
                value: OPTIMIZED_VALUE.to_string(),
                opcode: match (incs as i64 - decs as i64) > 0 {
                    true => Opcode::Inc,
                    false => Opcode::Dec,
                },
                argument: Some(match (incs as i64 - decs as i64) > 0 {
                    true => incs - decs,
                    false => decs - incs,
                }),
                ..fst
            });
            return true;
        }
        false
    }

    fn optimize_inc_dec_ptr_chains(&mut self, state: &mut OptimizerPassState) -> bool {
        let mut incs = 0;
        let mut decs = 0;
        let fst = state.peek_at(self, 0).unwrap();
        while state.can_advance(0) {
            let current = state.peek_at(self, 0);
            match current {
                Some(val) => {
                    match val.opcode {
                        Opcode::IncPtr => incs += val.argument.unwrap_or(1),
                        Opcode::DecPtr => decs += val.argument.unwrap_or(1),
                        _ => break,
                    }
                    state.skip(1);
                }
                None => break,
            }
        }
        if incs != 0 || decs != 0 {
            self.out_instructions.push(Instruction {
                value: OPTIMIZED_VALUE.to_string(),
                opcode: match (incs as i64 - decs as i64) > 0 {
                    true => Opcode::IncPtr,
                    false => Opcode::DecPtr,
                },
                argument: Some(match (incs as i64 - decs as i64) > 0 {
                    true => incs - decs,
                    false => decs - incs,
                }),
                ..fst
            });
            return true;
        }
        false
    }

    fn create_pass_state(&self) -> OptimizerPassState {
        OptimizerPassState {
            pos: 0,
            size: self.instructions.len(),
        }
    }
}

/// The `OptimizerPassState` implementation.
impl OptimizerPassState {
    /// Tests if reading `n` more characters is possible.
    fn can_advance(&self, n: usize) -> bool {
        self.pos + n < self.size
    }

    /// Peeks at the 'n'-th instruction relative to `self.pos`.
    fn peek_at(&self, optimizer: &mut Optimizer, n: usize) -> Option<Instruction> {
        match self.can_advance(n) {
            true => Some(optimizer.instructions[self.pos + n].clone()),
            false => None,
        }
    }

    /// Peeks at the next `n` instructions.
    fn peek(&self, optimizer: &mut Optimizer, n: usize) -> Option<Vec<Instruction>> {
        match self.can_advance(n) {
            true => {
                let mut vec: Vec<Instruction> = Vec::with_capacity(n);
                vec.extend_from_slice(&optimizer.instructions[self.pos..(self.pos + n)]);
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
