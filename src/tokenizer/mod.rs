pub mod parser;
pub mod tokenizer;

#[cfg(test)]
mod tests {
    use crate::tokenizer::parser::parse;

    fn check_same(str_tokens: &[String], expected: &[&str]) {
        if str_tokens.len() != expected.len() {
            panic!("Not the same length: result: {:?}, expected: {:?}", str_tokens, expected);
        }
        for (i, &r) in expected.iter().enumerate() {
            if r != str_tokens[i].as_str() {
                panic!("Different token at token {i}: result: {:?}, expected: {:?}", str_tokens, expected);
            }
        }
    }


    #[test]
    fn string_tokens() {
        let content = "x = $101u16; x.1 = $'d'u16;";

        let expected = ["x", "=", "$101u16", ";", "x.1", "=", "$'d'u16", ";"];

        let str_tokens = match parse(content) {
            Ok(v) => v,
            Err(e) => panic!("{e}"),
        };
        check_same(&str_tokens, &expected);
    }
}