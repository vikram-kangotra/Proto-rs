use inkwell::values::BasicValueEnum;
use std::ops::{Add, Sub, Mul, Div, Rem, Neg};

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum IntValue {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
}

impl IntValue {

    pub fn is_zero(&self) -> bool {
        match self {
            IntValue::I8(value) => value == &0,
            IntValue::I16(value) => value == &0,
            IntValue::I32(value) => value == &0,
            IntValue::I64(value) => value == &0,
        }
    }
}

impl Neg for IntValue {
    type Output = IntValue;

    fn neg(self) -> IntValue {
        match self {
            IntValue::I8(value) => IntValue::I8(-value),
            IntValue::I16(value) => IntValue::I16(-value),
            IntValue::I32(value) => IntValue::I32(-value),
            IntValue::I64(value) => IntValue::I64(-value),
        }
    }
}

impl Add for IntValue {
    type Output = IntValue;

    fn add(self, other: IntValue) -> IntValue {
        match (self, other) {
            (IntValue::I8(left), IntValue::I8(right)) => IntValue::I8(left + right),
            (IntValue::I16(left), IntValue::I16(right)) => IntValue::I16(left + right),
            (IntValue::I32(left), IntValue::I32(right)) => IntValue::I32(left + right),
            (IntValue::I64(left), IntValue::I64(right)) => IntValue::I64(left + right),
            _ => panic!("Unexpected type"),
        }
    }
}

impl Sub for IntValue {
    type Output = IntValue;

    fn sub(self, other: IntValue) -> IntValue {
        match (self, other) {
            (IntValue::I8(left), IntValue::I8(right)) => IntValue::I8(left - right),
            (IntValue::I16(left), IntValue::I16(right)) => IntValue::I16(left - right),
            (IntValue::I32(left), IntValue::I32(right)) => IntValue::I32(left - right),
            (IntValue::I64(left), IntValue::I64(right)) => IntValue::I64(left - right),
            _ => panic!("Unexpected type"),
        }
    }
}

impl Mul for IntValue {
    type Output = IntValue;

    fn mul(self, other: IntValue) -> IntValue {
        match (self, other) {
            (IntValue::I8(left), IntValue::I8(right)) => IntValue::I8(left * right),
            (IntValue::I16(left), IntValue::I16(right)) => IntValue::I16(left * right),
            (IntValue::I32(left), IntValue::I32(right)) => IntValue::I32(left * right),
            (IntValue::I64(left), IntValue::I64(right)) => IntValue::I64(left * right),
            _ => panic!("Unexpected type"),
        }
    }
}

impl Div for IntValue {
    type Output = IntValue;

    fn div(self, other: IntValue) -> IntValue {
        match (self, other) {
            (IntValue::I8(left), IntValue::I8(right)) => IntValue::I8(left / right),
            (IntValue::I16(left), IntValue::I16(right)) => IntValue::I16(left / right),
            (IntValue::I32(left), IntValue::I32(right)) => IntValue::I32(left / right),
            (IntValue::I64(left), IntValue::I64(right)) => IntValue::I64(left / right),
            _ => panic!("Unexpected type"),
        }
    }
}

impl Rem for IntValue {
    type Output = IntValue;

    fn rem(self, other: IntValue) -> IntValue {
        match (self, other) {
            (IntValue::I8(left), IntValue::I8(right)) => IntValue::I8(left % right),
            (IntValue::I16(left), IntValue::I16(right)) => IntValue::I16(left % right),
            (IntValue::I32(left), IntValue::I32(right)) => IntValue::I32(left % right),
            (IntValue::I64(left), IntValue::I64(right)) => IntValue::I64(left % right),
            _ => panic!("Unexpected type"),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum IntegerValue<'ctx> {
    Int(IntValue),
    LLVMInt(inkwell::values::IntValue<'ctx>),
}

impl<'ctx> Into<IntegerValue<'ctx>> for IntValue {
    fn into(self) -> IntegerValue<'ctx> {
        IntegerValue::Int(self)
    }
}

impl<'ctx> Into<IntValue> for IntegerValue<'ctx> {
    fn into(self) -> IntValue {
        match self {
            IntegerValue::Int(value) => value,
            _ => panic!("Expected IntValue"),
        }
    }
}

impl<'ctx> Into<IntegerValue<'ctx>> for inkwell::values::IntValue<'ctx> {
    fn into(self) -> IntegerValue<'ctx> {
        IntegerValue::LLVMInt(self)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum FloatValue {
    F32(f32),
    F64(f64),
}

impl FloatValue {
    pub fn is_zero(&self) -> bool {
        match self {
            FloatValue::F32(value) => value == &0.0,
            FloatValue::F64(value) => value == &0.0,
        }
    }
}

impl Neg for FloatValue {
    type Output = FloatValue;

    fn neg(self) -> FloatValue {
        match self {
            FloatValue::F32(value) => FloatValue::F32(-value),
            FloatValue::F64(value) => FloatValue::F64(-value),
        }
    }
}

impl Add for FloatValue {
    type Output = FloatValue;

    fn add(self, other: FloatValue) -> FloatValue {
        match (self, other) {
            (FloatValue::F32(left), FloatValue::F32(right)) => FloatValue::F32(left + right),
            (FloatValue::F64(left), FloatValue::F64(right)) => FloatValue::F64(left + right),
            _ => panic!("Unexpected type"),
        }
    }
}

impl Sub for FloatValue {
    type Output = FloatValue;

    fn sub(self, other: FloatValue) -> FloatValue {
        match (self, other) {
            (FloatValue::F32(left), FloatValue::F32(right)) => FloatValue::F32(left - right),
            (FloatValue::F64(left), FloatValue::F64(right)) => FloatValue::F64(left - right),
            _ => panic!("Unexpected type"),
        }
    }
}

impl Mul for FloatValue {
    type Output = FloatValue;

    fn mul(self, other: FloatValue) -> FloatValue {
        match (self, other) {
            (FloatValue::F32(left), FloatValue::F32(right)) => FloatValue::F32(left * right),
            (FloatValue::F64(left), FloatValue::F64(right)) => FloatValue::F64(left * right),
            _ => panic!("Unexpected type"),
        }
    }
}

impl Div for FloatValue {
    type Output = FloatValue;

    fn div(self, other: FloatValue) -> FloatValue {
        match (self, other) {
            (FloatValue::F32(left), FloatValue::F32(right)) => FloatValue::F32(left / right),
            (FloatValue::F64(left), FloatValue::F64(right)) => FloatValue::F64(left / right),
            _ => panic!("Unexpected type"),
        }
    }
}

impl Rem for FloatValue {
    type Output = FloatValue;

    fn rem(self, other: FloatValue) -> FloatValue {
        match (self, other) {
            (FloatValue::F32(left), FloatValue::F32(right)) => FloatValue::F32(left % right),
            (FloatValue::F64(left), FloatValue::F64(right)) => FloatValue::F64(left % right),
            _ => panic!("Unexpected type"),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum FloatingValue<'ctx> {
    Float(FloatValue),
    LLVMFloat(inkwell::values::FloatValue<'ctx>),
}

impl<'ctx> Into<FloatingValue<'ctx>> for FloatValue {
    fn into(self) -> FloatingValue<'ctx> {
        FloatingValue::Float(self)
    }
}

impl<'ctx> Into<FloatingValue<'ctx>> for inkwell::values::FloatValue<'ctx> {
    fn into(self) -> FloatingValue<'ctx> {
        FloatingValue::LLVMFloat(self)
    }
}

impl<'ctx> Into<FloatValue> for FloatingValue<'ctx> {
    fn into(self) -> FloatValue {
        match self {
            FloatingValue::Float(value) => value,
            _ => panic!("Expected FloatValue"),
        }
    }
}

impl<'ctx> Into<FloatingValue<'ctx>> for IntegerValue<'ctx> {
    fn into(self) -> FloatingValue<'ctx> {
        match self {
            IntegerValue::Int(IntValue::I8(value)) => FloatingValue::Float(FloatValue::F32(value as f32)),
            IntegerValue::Int(IntValue::I16(value)) => FloatingValue::Float(FloatValue::F32(value as f32)),
            IntegerValue::Int(IntValue::I32(value)) => FloatingValue::Float(FloatValue::F32(value as f32)),
            IntegerValue::Int(IntValue::I64(value)) => FloatingValue::Float(FloatValue::F32(value as f32)),
            _ => panic!("Conversion not supported"),
        }
    }
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

    pub fn as_literal(&self) -> LiteralValue {
        match self {
            Value::Literal(l) => *l,
            _ => panic!("Expected literal value"),
        }
    }

    pub fn as_llvm_basic_value_enum(&self) -> BasicValueEnum<'ctx> {
        match self {
            Value::LLVMBasicValueEnum(bve) => *bve,
            _ => panic!("Expected BasicValueEnum"),
        }
    }
}

impl<'ctx> Into<Value<'ctx>> for bool {
    fn into(self) -> Value<'ctx> {
        Value::Literal(LiteralValue::Bool(self))
    }
}

impl<'ctx> Into<Value<'ctx>> for char {
    fn into(self) -> Value<'ctx> {
        Value::Literal(LiteralValue::Char(self))
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

impl<'ctx> Into<Value<'ctx>> for IntegerValue<'ctx> {
    fn into(self) -> Value<'ctx> {
        match self {
            IntegerValue::Int(i) => Value::Literal(LiteralValue::Int(i)),
            IntegerValue::LLVMInt(i) => Value::LLVMBasicValueEnum(i.into()),
        }
    }
}

impl<'ctx> Into<Value<'ctx>> for FloatingValue<'ctx> {
    fn into(self) -> Value<'ctx> {
        match self {
            FloatingValue::Float(f) => Value::Literal(LiteralValue::Float(f)),
            FloatingValue::LLVMFloat(f) => Value::LLVMBasicValueEnum(f.into()),
        }
    }
}

impl<'ctx> Into<inkwell::values::IntValue<'ctx>> for IntegerValue<'ctx> {
    fn into(self) -> inkwell::values::IntValue<'ctx> {
        match self {
            IntegerValue::Int(_) => panic!("Cannot convert IntValue to LLVMIntValue"),
            IntegerValue::LLVMInt(i) => i,
        }
    }
}

impl<'ctx> Into<inkwell::values::FloatValue<'ctx>> for FloatingValue<'ctx> {
    fn into(self) -> inkwell::values::FloatValue<'ctx> {
        match self {
            FloatingValue::Float(_) => panic!("Cannot convert FloatValue to LLVMFloatValue"),
            FloatingValue::LLVMFloat(f) => f,
        }
    }
}

impl<'ctx> Into<Value<'ctx>> for IntValue {
    fn into(self) -> Value<'ctx> {
        Value::Literal(LiteralValue::Int(self))
    }
}

impl<'ctx> Into<Value<'ctx>> for FloatValue {
    fn into(self) -> Value<'ctx> {
        Value::Literal(LiteralValue::Float(self))
    }
}
