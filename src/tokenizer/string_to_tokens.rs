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
    ParenOpen,
    ParenClose,
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
    EndLine,
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

fn process_line(s: &str, line: u64) -> Result<Vec<Token>, TokenizationError> {
    let mut tokens = Vec::new();
    let mut current_token = String::new();
    let mut s_iter = s.chars().enumerate();
    for (i, c) in s_iter {
        if c.is_whitespace() {
            let token = match current_token.as_str() {
                "fn" => Token::KeyWord(KeyWord::Fn),
                "endfn" => Token::KeyWord(KeyWord::EndFn),
                "let" => Token::KeyWord(KeyWord::Let),
                "const" => Token::KeyWord(KeyWord::Const),
                "call" => Token::KeyWord(KeyWord::Call),
                "jmp" => Token::KeyWord(KeyWord::Jmp),
                "label" => Token::KeyWord(KeyWord::Label),
                _ => return Err(TokenizationError::InvalidKeyword(line))
            };
            tokens.push(token);
            break;
        }
        current_token.push(c);
    }
    let kw = match tokens.get(0).ok_or(TokenizationError::InvalidKeyword(line))? {
        Token::KeyWord(kw) => kw,
        _ => return Err(TokenizationError::InvalidKeyword(line)),
    };
    match kw {
        KeyWord::Fn => {
            for (c, i) in s_iter {
                if c ==
            }
        },
        KeyWord::EndFn => {

        },
        KeyWord::Let => {

        },
        KeyWord::Const => {

        },
        KeyWord::Call => {

        },
        KeyWord::Jmp => {

        },
        KeyWord::Label => {

        },
    }
    Ok(tokens)
}

fn tokenize_from_str(s: &str) -> Result<Vec<TokenObject>, TokenizationError> {
    let mut tokens = Vec::new();

    for (li, line) in s.lines().enumerate() {
        let mut skip_amount = 0;
        let mut current_line = Vec::new();
        let mut current_token = String::new();
        for (i, kwc) in line.chars().enumerate() {
            if kwc.is_whitespace() {
                let token = match current_token.as_str() {
                    "fn" => Token::KeyWord(KeyWord::Fn),
                    "endfn" => Token::KeyWord(KeyWord::EndFn),
                    "let" => Token::KeyWord(KeyWord::Let),
                    "const" => Token::KeyWord(KeyWord::Const),
                    "call" => Token::KeyWord(KeyWord::Call),
                    _ => return Err(TokenizationError::InvalidKeyword(li as u64))
                };
                skip_amount = i;
                current_line.push(token);
                break;
            }
            current_token.push(kwc);
        }
    }

    Ok(tokens)
}