mod tokenizer;

use std::fs;

use crate::tokenizerlexer::parser::parse;

fn main() {
    let content_bytes = match fs::read("src/test.hvl") {
        Ok(v) => v,
        Err(e) => {eprintln!("Error occured in file read: {e}"); return}
    };

    let content = match str::from_utf8(&content_bytes) {
        Ok(s) => s,
        Err(_) => {eprintln!("File was not valid UTF8"); return}
    };

    let str_tokens = match parse(content) {
        Ok(v) => v,
        Err(e) => {eprintln!("Error parsing input: {e}"); return},
    };

    println!("{content}\n{str_tokens:?}");
}