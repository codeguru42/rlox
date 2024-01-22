use prelude::*;

mod chunk;
mod debug;

mod prelude {
    pub use crate::chunk::*;
}

fn main() {
    let mut chunk = Chunk::new();
    chunk.write(chunk::OpCode::OpReturn as u8);
    chunk.disassemble("test chunk");
}
