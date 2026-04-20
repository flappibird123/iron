use super::opcodes::*;
use crate::frontend::chunk::Chunk;
use std::convert::TryInto;

pub struct VM<'a> {
    ip: usize,
    stack: Vec<i64>,
    chunk: &'a Chunk,
    locals: Vec<i64>,
    end: usize,
}

impl<'a> VM<'a> {
    pub fn new(chunk: &'a Chunk) -> Self {
        Self {
            ip: 0,
            stack: Vec::new(),
            chunk,
            locals: vec![0; 256],
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
                OpCode::StoreLocal => {
                    let index = self.chunk.bytecode.get(self.ip + 1).unwrap_or_else(|| panic!("missing operand of storelocal")).clone();
                    self.ip += 1;
                    let value = self.pop();
                    self.locals[index as usize] = value;
                },
                OpCode::LoadLocal => {
                    let index = self.chunk.bytecode.get(self.ip + 1).unwrap_or_else(|| panic!("missing operand of storelocal")).clone();
                    self.ip += 1;
                    let value = self.locals[index as usize];
                    self.push(value);
                }
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
            panic!("stack underflow");
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
            0x07 => OpCode::StoreLocal,
            0x08 => OpCode::LoadLocal,
            _ => panic!("Encountered unknown opcode: {}", byte)
        }
    }
}