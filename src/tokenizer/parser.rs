use std::{error::Error, fmt::Display};

use crate::tokenizerlexer::Line;

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

pub fn parse(s: &str) -> Result<Vec<(String, Line)>, ParseError> {
    let mut tokens = Vec::new();

    let mut line: Line = Line(1);

    let mut current_token = String::new();

    if !s.is_ascii() {
        return Err(ParseError::NonAsciiContent)
    }

    for b in s.bytes() {
        let c = b as char;
        if c == '\n' {
            line.inc();
        }
        if c.is_whitespace() {
            if !current_token.is_empty() {
                tokens.push((current_token, line));
                current_token = String::new();
            }
        }
        else if !c.is_ascii_alphanumeric() && c != '_' && c != '$' && c != '.' && c != '\'' {
            if !current_token.is_empty() {
                tokens.push((current_token, line));
                current_token = String::new();
            }
            tokens.push((c.to_string(), line));
        }
        else {
            current_token.push(c);
        }
    }
    if !current_token.is_empty() {
        tokens.push((current_token, line));
    }

    Ok(tokens)
}