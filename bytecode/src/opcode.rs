#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum OpCode {
    Constant = 0,
    Negate,
    Add,
    Subtract,
    Multiply,
    Divide,
    Return,
}

impl OpCode {
    #[inline]
    pub fn read(byte: u8) -> Self {
        // SAFETY: only called on bytes emitted by Chunk::emit_op,
        // which takes OpCode and casts to u8
        unsafe { std::mem::transmute(byte) }
    }
}
