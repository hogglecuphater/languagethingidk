mod tokenizer;

use std::fs;

use crate::tokenizer::parser::parse;

fn main() {
    let content_bytes = match fs::read("src/test.hvl") {
        Ok(v) => v,
        Err(e) => {eprintln!("Error occured in file read: {e}"); return}
    };

    let content = match str::from_utf8(&content_bytes) {
        Ok(s) => s,
        Err(_) => {eprintln!("File was not valid UTF8"); return}
    };

    let str_tokens = parse(content);

    println!("{content}\n{str_tokens:?}");
}