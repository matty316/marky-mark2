use std::fs;
use crate::lexer::Lexer;

pub fn marky_str(source: &str) {
    let lexer = Lexer::new(source);

    lexer.scan();
}

pub fn marky(path: &str) {
    let contents = fs::read_to_string(path);

    match contents {
        Ok(c) => marky_str(c.as_str()),
        Err(err) => eprintln!("{:?}", err),
    }
}
