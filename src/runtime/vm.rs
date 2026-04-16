use super::opcodes::*;
use std::convert::TryInto;

pub struct VM<'a> {
    ip: usize,
    stack: Vec<i64>,
    chunk: &'a Module,
    end: usize,
}

pub struct Module {
    pub bytecode: Vec<u8>,
    pub constants: Vec<i64>,
}

impl Module {
    pub fn new(bytecode: Vec<u8>, constants: Vec<i64>) -> Self {
        Self {
            bytecode,
            constants
        }
    }
}

impl<'a> VM<'a> {
    pub fn new(chunk: &'a Module) -> Self {
        Self {
            ip: 0,
            stack: Vec::new(),
            chunk,
            end: chunk.bytecode.len()
        }
    }    

    pub fn run(&mut self) -> i32 {
        while self.ip < self.end {
            let byte = self.fetch();
            let opcode = self.decode(byte);
            match opcode {
                OpCode::Print => {
                    let val = self.pop();
                    println!("{}", val);
                },
                OpCode::Add => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(a + b);
                },
                OpCode::Sub => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(a - b);
                },
                OpCode::Mul => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(a * b);
                },
                OpCode::Div => {
                    let b = self.pop();
                    if b == 0 {
                        panic!("Division by 0");
                    }
                    let a = self.pop();
                    self.push(a / b);
                },
                OpCode::Hlt => {
                    return 0;
                },
                OpCode::LoadConst => {
                    let index = self.chunk.bytecode.get(self.ip + 1).unwrap_or_else(|| panic!("missing operand of loadconst")).clone();
                    let index: usize = index.try_into().expect("constant index cannot be negative");
                    self.ip += 1;
                    let val = self.chunk.constants.get(index).unwrap_or_else(|| panic!("invalid constant pool")).clone();
                    self.push(val);
                },
            }
            self.ip += 1;
        }
        1 // 0 indicates success. 1 indicates failure
    }

    fn push(&mut self, value: i64) {
        self.stack.push(value);
    }

    fn pop(&mut self) -> i64 {
        if let Some(v) = self.stack.pop() {
            return v;
        } else {
            panic!("pop from empty stack");
        }
    }

    fn fetch(&self) -> u8 {
        self.chunk.bytecode[self.ip]
    }

    fn decode(&self, byte: u8) -> OpCode {
        return match byte {
            0x00 => OpCode::Print,
            0x01 => OpCode::Add,
            0x02 => OpCode::Sub,
            0x03 => OpCode::Mul,
            0x04 => OpCode::Div,
            0x05 => OpCode::LoadConst,
            0x06 => OpCode::Hlt,
            _ => panic!("Encountered unknown opcode: {}", byte)
        }
    }
}