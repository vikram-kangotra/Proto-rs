#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum IntType {
    U8,
    U16,
    U32,
    U64,

    I8,
    I16,
    I32,
    I64,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum FloatType {
    F32,
    F64,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum LiteralType {
    Bool,
    Char,
    Int(IntType),
    Float(FloatType),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Type {
    Literal(LiteralType),
    Array(Box<Type>, usize),
    Inferred,
    Void
}
