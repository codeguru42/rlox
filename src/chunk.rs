use int_enum::IntEnum;

#[repr(u8)]
#[derive(IntEnum)]
pub enum OpCode {
    OpReturn = 0,
}

pub struct Chunk {
    pub(crate) code: Vec<u8>,
}

impl Chunk {
    pub(crate) fn new() -> Self {
        Self { code: Vec::new() }
    }

    pub(crate) fn write(&mut self, byte: u8) {
        self.code.push(byte);
    }
}
