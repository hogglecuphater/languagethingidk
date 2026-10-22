use std::fmt::Display;

pub mod parser;
pub mod tokenizer;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Line(usize);

impl Line {
    pub fn inc(&mut self) {
        self.0 += 1;
    }
}

impl Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}