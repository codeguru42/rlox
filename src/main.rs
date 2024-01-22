use prelude::*;

mod chunk;
mod debug;
mod value;

mod prelude {
    pub use crate::chunk::*;
    pub use crate::value::*;
}

fn main() {
    let mut chunk = Chunk::new();
    let constant = chunk.add_constant(1.2);
    chunk.write(OpCode::OpConstant as u8, 123);
    chunk.write(constant, 123);
    chunk.write(chunk::OpCode::OpReturn as u8, 123);
    chunk.disassemble("test chunk");
}
