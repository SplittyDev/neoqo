mod vm;

use std::cell::Cell;
use vm::opcodes::Opcode;

pub struct Lexer {
    stream: Vec<char>,
    tokens: Vec<Lexeme>,
    stream_pos: Cell<i32>,
    current_pos: Cell<i32>,
    current_line: Cell<i32>,
}

pub struct Lexeme {
    pos: i32,
    line: i32,
    value: String,
    opcode: Opcode,
}

impl Lexer {
    pub fn new(stream: String) -> Lexer {
        Lexer {
            stream: stream.chars().collect(),
            stream_pos: Cell::new(-1),
            current_pos: Cell::new(0),
            current_line: Cell::new(0),
            tokens: Vec::new(),
        }
    }

    pub fn tokenize(&mut self) {
        let operators = String::from("<>[]()+-*/\\.,:;&^#=@bciqx!");
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
                       Opcode::Switch,
                       Opcode::Print,
                       Opcode::Read,
                       Opcode::Push,
                       Opcode::Pop,
                       Opcode::Dup,
                       Opcode::JmpStack,
                       Opcode::Count,
                       Opcode::Compare,
                       Opcode::Reverse,
                       Opcode::BinMod,
                       Opcode::ChrMod,
                       Opcode::IntMod,
                       Opcode::Terminate,
                       Opcode::HexMod,
                       Opcode::Break];
        while self.can_advance(1) {
            self.skip_whitespace();
            if !self.can_advance(1) {
                break;
            }
            let chr = self.peek(1);
            let state = (self.current_pos.get(), self.current_line.get());
            if operators.contains(chr) {
                self.skip(1);
                let val = chr.to_string();
                let op = opcodes[operators.chars().position(|x| x == chr).unwrap()];
                self.create_lexeme(state, val, op);
                continue;
            }
            match chr {
                '"' => {
                    self.skip(1);
                    let mut buf = String::with_capacity(32);
                    while self.can_advance(1) && self.peek(1) != '"' {
                        buf.push(self.peek(1));
                        self.skip(1);
                    }
                    self.skip(1);
                    self.create_lexeme(state, buf, Opcode::Str);
                }
                '\'' => {
                    while self.can_advance(1) && self.peek(1) != '\n' {
                        self.skip(1);
                    }
                    self.skip(1);
                }
                _ => (),
            };
        }
    }

    fn can_advance(&self, n: i32) -> bool {
        self.stream_pos.get() + n < self.stream.len() as i32
    }

    fn peek(&mut self, n: i32) -> char {
        match self.can_advance(n) {
            true => self.stream[(self.stream_pos.get() + n) as usize],
            false => '\0',
        }
    }

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

    fn skip_whitespace(&mut self) {
        while self.peek(1).is_whitespace() {
            self.skip(1);
        }
    }

    fn create_lexeme(&mut self, state: (i32, i32), value: String, opcode: Opcode) {
        println!("[{}:{}] '{}'", state.1, state.0, value);
        let lex = Lexeme {
            pos: state.0,
            line: state.1,
            value: value,
            opcode: opcode,
        };
        self.tokens.push(lex);
    }
}
