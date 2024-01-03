use inkwell::values::BasicValueEnum;

#[derive(Debug, Copy, Clone)]
pub enum IntValue {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
}

#[derive(Debug, Copy, Clone)]
pub enum FloatValue {
    F32(f32),
    F64(f64),
}

#[derive(Debug, Copy, Clone)]
pub enum LiteralValue {
    Bool(bool),
    Char(char),
    Int(IntValue),
    Float(FloatValue),
}

#[derive(Debug)]
pub enum Value<'ctx> {
    Literal(LiteralValue),
    LLVMBasicValueEnum(BasicValueEnum<'ctx>),
}

impl<'ctx> Value<'ctx> {
    pub fn as_bool(&self) -> bool {
        match self {
            Value::Literal(LiteralValue::Bool(b)) => *b,
            _ => panic!("Expected bool value"),
        }
    }

    pub fn as_char(&self) -> char {
        match self {
            Value::Literal(LiteralValue::Char(c)) => *c,
            _ => panic!("Expected char value"),
        }
    }

    pub fn as_int(&self) -> IntValue {
        match self {
            Value::Literal(LiteralValue::Int(i)) => *i,
            _ => panic!("Expected int value"),
        }
    }

    pub fn as_float(&self) -> FloatValue {
        match self {
            Value::Literal(LiteralValue::Float(f)) => *f,
            _ => panic!("Expected float value"),
        }
    }

    pub fn as_llvm_basic_value_enum(&self) -> BasicValueEnum<'ctx> {
        match self {
            Value::LLVMBasicValueEnum(bve) => *bve,
            _ => panic!("Expected BasicValueEnum"),
        }
    }
}

impl<'ctx> Into<Value<'ctx>> for LiteralValue {
    fn into(self) -> Value<'ctx> {
        Value::Literal(self)
    }
}

impl<'ctx> Into<Value<'ctx>> for BasicValueEnum<'ctx> {
    fn into(self) -> Value<'ctx> {
        Value::LLVMBasicValueEnum(self)
    }
}

impl<'ctx> Into<Value<'ctx>> for inkwell::values::IntValue<'ctx> {
    fn into(self) -> Value<'ctx> {
        Value::LLVMBasicValueEnum(self.into())
    }
}

impl<'ctx> Into<Value<'ctx>> for inkwell::values::FloatValue<'ctx> {
    fn into(self) -> Value<'ctx> {
        Value::LLVMBasicValueEnum(self.into())
    }
}
