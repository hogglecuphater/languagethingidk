use std::{error::Error, fmt::Display};

use crate::tokenizerlexer::Line;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyWord {
    Fn,
    Ret,
    Call,
    Push,
    Load,
    Store,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operation {
    Keep,
    CurrentLable,

    Add,
    Sub,
    Mul,
    Div,

    As,

    LeftShift,
    RightShift,

    Not,
    And,
    Or,
    Xor,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Bool,
    U8,
    U16,
    U32,
    U64,
    U128,
    I8,
    I16,
    I32,
    I64,
    I128,
    F32,
    F64,
    Executable,
    Ptr(Box<Type>),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Literal {
    Bool(bool),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    F32(f32),
    F64(f64),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Symbol {
    EndLine,
    Equal,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Keyword(KeyWord),
    Operation(Operation),
    Type(Type),
    Literal(Literal),
    Symbol(Symbol),
    VReg(String),
    FunctionName(String),
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenizeError {
    
}

impl Display for TokenizeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // match self {
            // Self::NonAsciiContent => write!(f, "non-ascii content tried to parse")
        // }
        Ok(())
    }
}

impl Error for TokenizeError {}

pub fn generate_tokens(str_tokens: Vec<(String, Line)>) -> Result<Vec<(Token, Line)>, TokenizeError> {
    let mut tokens = Vec::new();

    for (s, l) in str_tokens {
        // Keywords
        if match s.as_str() {
            "fn" => {tokens.push((Token::Keyword(KeyWord::Fn), l)); true},
            "ret" => {tokens.push((Token::Keyword(KeyWord::Ret), l)); true},
            "call" => {tokens.push((Token::Keyword(KeyWord::Call), l)); true},
            "push" => {tokens.push((Token::Keyword(KeyWord::Push), l)); true},
            "load" => {tokens.push((Token::Keyword(KeyWord::Load), l)); true},
            "store" => {tokens.push((Token::Keyword(KeyWord::Store), l)); true},
            _ => false,
        } {}
    }

    Ok(tokens)
}