use crate::prelude::{Chunk, OpCode};

impl Chunk {
    pub(crate) fn disassemble(&self, name: &str) {
        println!("== {} ==", name);

        let mut offset = 0;
        while offset < self.code.len() {
            offset = self.disassemble_instruction(offset);
        }
    }
    fn disassemble_instruction(&self, offset: usize) -> usize {
        print!("{} ", offset);

        let instruction = self.code[offset];
        return match OpCode::try_from(instruction) {
            Ok(OpCode::OpReturn) => simple_instruction("OP_RETURN", offset),
            _ => {
                println!("Unkown opcode {}", instruction);
                offset + 1
            }
        };
    }
}

fn simple_instruction(name: &str, offset: usize) -> usize {
    println!("{}", name);
    offset + 1
}
