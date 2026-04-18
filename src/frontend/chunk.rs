
#[derive(Debug)]
pub struct Chunk {
    pub bytecode: Vec<u8>,
    pub constants: Vec<i64>,
}

impl Chunk {
    pub fn new(bytecode: Vec<u8>, constants: Vec<i64>) -> Self {
        Self {
            bytecode,
            constants
        }
    }
}
