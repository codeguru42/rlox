use crate::prelude::{Chunk, OpCode, Value};

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
            Ok(OpCode::OpConstant) => constant_instruction("OP_CONSTANT", self, offset),
            _ => {
                println!("Unkown opcode {}", instruction);
                offset + 1
            }
        };
    }
}

fn constant_instruction(name: &str, chunk: &Chunk, offset: usize) -> usize {
    let constant = chunk.code[offset + 1] as usize;
    print!("{} {} ", name, constant);
    print_value(chunk.constants[constant]);
    println!();
    offset + 2
}

fn print_value(value: Value) {
    print!("{}", value)
}

fn simple_instruction(name: &str, offset: usize) -> usize {
    println!("{}", name);
    offset + 1
}
