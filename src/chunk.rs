use crate::value::Value;
use int_enum::IntEnum;

#[repr(u8)]
#[derive(IntEnum)]
pub enum OpCode {
    OpConstant = 0,
    OpReturn = 1,
}

pub struct Chunk {
    pub(crate) code: Vec<u8>,
    pub(crate) constants: Vec<Value>,
}

impl Chunk {
    pub(crate) fn new() -> Self {
        Self {
            code: Vec::new(),
            constants: Vec::new(),
        }
    }

    pub(crate) fn write(&mut self, byte: u8) {
        self.code.push(byte);
    }

    pub(crate) fn add_constant(&mut self, value: Value) -> u8 {
        self.constants.push(value);
        (self.constants.len() - 1) as u8
    }
}
