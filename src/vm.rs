use int_enum::IntEnum;

use crate::chunk::OpCode;
use crate::debug::print_value;
use crate::prelude::Chunk;

#[repr(u8)]
#[derive(IntEnum)]
pub(crate) enum InterpretResult {
    InterpretOk = 1,
}

pub struct VM {
    chunk: Chunk,
    ip: usize,
}

impl VM {
    pub(crate) fn new() -> Self {
        Self {
            chunk: Chunk::new(),
            ip: 0,
        }
    }

    pub(crate) fn interpret(&mut self, chunk: Chunk) -> InterpretResult {
        self.chunk = chunk;
        self.ip = 0;
        self.run()
    }
    fn run(&mut self) -> InterpretResult {
        loop {
            let instruction = self.get_byte();

            match OpCode::try_from(instruction) {
                Ok(OpCode::OpReturn) => return InterpretResult::InterpretOk,
                Ok(OpCode::OpConstant) => {
                    let idx = self.get_byte() as usize;
                    let constant = self.chunk.constants[idx];
                    print_value(constant);
                }
                _ => {}
            }
        }
    }

    fn get_byte(&mut self) -> u8 {
        let byte = self.chunk.code[self.ip];
        self.ip += 1;
        byte
    }
}
