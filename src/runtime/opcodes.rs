
#[derive(Debug)]
#[repr(u8)]
pub enum OpCode {
    Print = 0x00,

    Add = 0x01,
    Sub = 0x02,
    Mul = 0x03,
    Div = 0x04,

    LoadConst = 0x05, // expect operand that is 1 byte for now. maybe will be 16bits in the future stored in little endian

    Hlt = 0x06,

}