use crate::lexer::Lexer;
use crate::token::TokenType;
use std::io::{stdin, stdout, Write};

const PROMPT: &str = ">> ";

pub fn start() {
    let mut line = String::new();
    loop {
        print!("{PROMPT}");
        stdout().flush().unwrap();
        let len = stdin().read_line(&mut line).expect("Failed to read");
        match len {
            0 => break,
            _ => {
                let mut l = Lexer::new(line.as_ref());
                loop {
                    let tok = l.next_token();
                    match tok.token_type {
                        TokenType::EOF => break,
                        _ => println!("{:?}", tok),
                    }
                }
                line = String::new();
            }
        }
    }
}
