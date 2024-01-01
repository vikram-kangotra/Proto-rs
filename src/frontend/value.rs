#[derive(Copy, Clone)]
pub enum IntValue {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
}

#[derive(Copy, Clone)]
pub enum FloatValue {
    F32(f32),
    F64(f64),
}

#[derive(Copy, Clone)]
pub enum LiteralValue {
    Bool(bool),
    Char(char),
    Int(IntValue),
    Float(FloatValue),
}
