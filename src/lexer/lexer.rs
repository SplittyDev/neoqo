use std::cell::Cell;
use vm::opcodes::Opcode;
use vm::instruction::Instruction;

/// The `Lexer` type.
pub struct Lexer {
    /// The character stream that represents the source
    stream: Vec<char>,

    /// The current position in the stream
    stream_pos: Cell<i32>,

    /// The current position on the current line
    current_pos: Cell<i32>,

    /// The current line
    current_line: Cell<i32>,

    /// A collection of instructions
    ///
    /// The collection is populated by the lexer while processing the source.
    pub tokens: Vec<Instruction>,
}

/// The `Lexer` implementation.
impl Lexer {
    /// Constructs a new `Lexer`.
    pub fn new(stream: String) -> Lexer {

        // Create the lexer
        Lexer {
            stream: stream.chars().collect(),
            stream_pos: Cell::new(-1),
            current_pos: Cell::new(0),
            current_line: Cell::new(0),
            tokens: Vec::new(),
        }
    }

    /// Tokenizes the source collection.
    pub fn tokenize(&mut self) {

        // A string containing all valid operators
        let operators = String::from("<>[]()+-*/\\.,:;&^#=bciqx!");

        // A fixed array containing all valid opcodes
        // The elements of the array match the operators in the above string.
        let opcodes = [Opcode::DecPtr,
                       Opcode::IncPtr,
                       Opcode::JzCell,
                       Opcode::JnzCell,
                       Opcode::JzStack,
                       Opcode::JnzStack,
                       Opcode::Inc,
                       Opcode::Dec,
                       Opcode::Double,
                       Opcode::Halve,
                       Opcode::Swap,
                       Opcode::Print,
                       Opcode::Read,
                       Opcode::Push,
                       Opcode::Pop,
                       Opcode::Dup,
                       Opcode::JmpStack,
                       Opcode::Count,
                       Opcode::Compare,
                       Opcode::BinMod,
                       Opcode::ChrMod,
                       Opcode::IntMod,
                       Opcode::Terminate,
                       Opcode::HexMod,
                       Opcode::Break];

        // Loop until there are no characters left
        while self.can_advance(1) {

            // Eat whitespace characters
            self.skip_whitespace();

            // Double-check if we can still advance in the source
            // after stripping whitespace characters.
            if !self.can_advance(1) {
                break;
            }

            // Peek at the next character
            let chr = self.peek(1);

            // Remember the current line and position for later use
            let state = (self.current_pos.get(), self.current_line.get());

            // Test if the operator is a single-character operator
            // and is contained in the `operators` string.
            if operators.contains(chr) {

                // Skip the current character
                self.skip(1);

                // Create an instruction from the current character
                let val = chr.to_string();
                let op = opcodes[operators.chars().position(|x| x == chr).unwrap()];
                self.create_instruction(state, val, op);

                // Continue looping
                continue;
            }

            // Match the current character against a few special cases
            match chr {

                // Test if the character is part of a string literal
                '"' => {
                    self.skip(1);
                    let mut buf = String::with_capacity(32);
                    while self.can_advance(1) && self.peek(1) != '"' {
                        buf.push(self.peek(1));
                        self.skip(1);
                    }
                    self.skip(1);
                    self.create_instruction(state, buf, Opcode::Str);
                }

                // Test if the character is part of a comment
                '\'' => {
                    while self.can_advance(1) && self.peek(1) != '\n' {
                        self.skip(1);
                    }
                    self.skip(1);
                }

                // Ignore all other cases
                _ => (),
            };
        }
    }

    /// Tests if reading `n` more characters is possible.
    fn can_advance(&self, n: i32) -> bool {
        self.stream_pos.get() + n < self.stream.len() as i32
    }

    /// Peeks at the character at relative position `n`.
    ///
    /// Returns an ASCII `NUL` character (`'\0'`) if reading `n` more
    /// characters is not possible.
    fn peek(&mut self, n: i32) -> char {
        match self.can_advance(n) {
            true => self.stream[(self.stream_pos.get() + n) as usize],
            false => '\0',
        }
    }

    /// Skips the next `n` characters.
    ///
    /// Keeps track of the current line and position.
    fn skip(&mut self, n: i32) {
        let mut i = 0;
        while i < n {
            let (pos, line) = match self.peek(1) {
                '\n' => (0, self.current_line.get() + 1),
                _ => (self.current_pos.get() + 1, self.current_line.get()),
            };
            self.current_pos.set(pos);
            self.current_line.set(line);
            self.stream_pos.set(self.stream_pos.get() + 1);
            i += 1;
        }
    }

    /// Skips whitespace characters.
    fn skip_whitespace(&mut self) {
        while self.peek(1).is_whitespace() {
            self.skip(1);
        }
    }

    /// Creates an instruction and adds it to the collection.
    ///
    /// The state parameter expects a tuple contaning the position relative to the current line
    /// and the current line, in that order.
    fn create_instruction(&mut self, state: (i32, i32), value: String, opcode: Opcode) {
        let lex = Instruction::new(state.0, state.1, value, opcode);
        self.tokens.push(lex);
    }
}
