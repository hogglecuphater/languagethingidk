use std::{error::Error, fmt::Display};

use crate::tokenizer::Line;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyWords {
    Fn,
    Ret,
    Call,
    Push,
    Load,
    Store,
}

pub enum Operations {
    Keep,

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
    TypeLiteral,
    Ptr(Box<Type>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Keyword(KeyWords)
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
    let tokens = Vec::new();

    Ok(tokens)
}