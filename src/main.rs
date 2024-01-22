use prelude::*;

mod chunk;
mod debug;
mod value;
mod vm;

mod prelude {
    pub(crate) use crate::chunk::*;
    pub(crate) use crate::value::*;
    pub(crate) use crate::vm::*;
}

fn main() {
    let mut vm = VM::new();
    let mut chunk = Chunk::new();
    let constant = chunk.add_constant(1.2);
    chunk.write(OpCode::OpConstant as u8, 123);
    chunk.write(constant, 123);
    chunk.write(chunk::OpCode::OpReturn as u8, 123);
    chunk.disassemble("test chunk");
    vm.interpret(chunk);
}
