use std::{error::Error, fmt::Display};

use crate::tokenizer::string_to_tokens::TokenizationError::InvalidKeyword;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum KeyWord {
    Fn,
    EndFn,
    Let,
    Const,
    Call,
    Jmp,
    Label,
}

#[derive(Debug, Clone, PartialEq)]
enum Literal {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    F32(f32),
    F64(f64),
    US(usize),
    IS(isize),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Type {
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    F32,
    F64,
    US,
    IS,
    Ptr(Box<Type>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    And,
    Or,
    Xor,
    Not,
    As,
    Deref,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Symbol {
    Equal,
    EndLine,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct RegID(pub u64);

#[derive(Debug, Clone, PartialEq)]
enum Token {
    KeyWord(KeyWord),
    Literal(Literal),
    Type(Type),
    Operator(Operator),
    Symbol(Symbol),
    Register(RegID),
}

#[derive(Debug, Clone, PartialEq)]
struct TokenObject {
    token: Token,
    line: u64,
}

impl TokenObject {
    pub fn new(token: Token, line: u64) -> Self {
        Self { token, line }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TokenizationError {
    InvalidKeyword(u64),
}

impl Display for TokenizationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InvalidKeyword(line) => write!(f, "invalid keyword was found at line {line}")
        }
    }
}

impl Error for TokenizationError {}

fn tokenize_to_str(s: String) -> Vec<String> {
    let mut tokens = Vec::new();

    let mut current_token = String::new();
    for c in s.chars() {
        if c == ' ' {
            if !tokens.is_empty() {
                tokens.push(current_token);
                current_token = String::new();
            }
            else if {
                
            }
        }
    }

    tokens
}