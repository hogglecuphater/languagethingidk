use std::{error::Error, fmt::Display};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseError {
    NonAsciiContent
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NonAsciiContent => write!(f, "non-ascii content tried to parse")
        }
    }
}

impl Error for ParseError {}

pub fn parse(s: &str) -> Result<Vec<String>, ParseError> {
    let mut tokens = Vec::new();

    let mut _line: usize = 1;

    let mut current_token = String::new();

    if !s.is_ascii() {
        return Err(ParseError::NonAsciiContent)
    }

    for b in s.bytes() {
        let c = b as char;
        if c == ' ' {
            if !current_token.is_empty() {
                tokens.push(current_token);
                current_token = String::new();
            }
        }
        else if !c.is_ascii_alphanumeric() && c != '_' && c != '$' && c != '.' && c != '\'' {
            if !current_token.is_empty() {
                tokens.push(current_token);
                current_token = String::new();
            }
            tokens.push(c.to_string());
        }
        else {
            current_token.push(c);
        }
    }
    if !current_token.is_empty() {
        tokens.push(current_token);
    }

    Ok(tokens)
}