use crate::token::token::{Token, TokenType};
use std::str::FromStr;

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: Option<char>,
}

impl Lexer {
    pub fn new(input: &str) -> Lexer {
        let mut l = Self {
            input: input.to_string(),
            position: 0,
            read_position: 0,
            ch: None,
        };
        l.read_char();
        l
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        if self.ch == None {
            return Token {
                token_type: TokenType::EOF,
                literal: "EOF".to_string(),
            };
        }

        let current_ch = self.ch.unwrap();
        let literal = current_ch.to_string();

        if current_ch == '=' && self.peek_char() == Some('=') {
            self.read_char();
            self.read_char();
            return Token {
                token_type: TokenType::EQ,
                literal: "==".to_string(),
            };
        }

        if current_ch == '!' && self.peek_char() == Some('=') {
            self.read_char();
            self.read_char();
            return Token {
                token_type: TokenType::NEQ,
                literal: "!=".to_string(),
            };
        }

        match TokenType::from_str(&literal) {
            Ok(token_type) => {
                self.read_char();
                Token {
                    token_type,
                    literal,
                }
            }
            _ => {
                if current_ch.is_alphabetic() {
                    self.read_identifier()
                } else if current_ch.is_numeric() {
                    self.read_number()
                } else {
                    Token {
                        token_type: TokenType::ILLEGAL,
                        literal,
                    }
                }
            }
        }
    }

    fn read_identifier(&mut self) -> Token {
        let pos = self.position;
        while self.ch.unwrap().is_alphabetic() {
            self.read_char();
        }
        let literal = self.input.get(pos..self.position).unwrap().to_string();
        Token {
            token_type: TokenType::lookup_identifier(&literal),
            literal,
        }
    }

    fn read_number(&mut self) -> Token {
        let pos = self.position;
        while self.ch.unwrap().is_numeric() {
            self.read_char();
        }
        let literal = self.input.get(pos..self.position).unwrap().to_string();
        Token {
            token_type: TokenType::INT,
            literal,
        }
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = None;
        } else {
            self.ch = self.input.chars().nth(self.read_position);
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn peek_char(&mut self) -> Option<char> {
        if self.read_position >= self.input.len() {
            None
        } else {
            self.input.chars().nth(self.read_position)
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(cch) = self.ch {
            if cch.is_whitespace() {
                self.read_char();
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_token() {
        let input = "\
            let five = 5;
            let ten = 10;
            let add = fn(x, y) {
                x+y;
            };
            let result = add(five, ten);

            !-/*5;
            5 < 10 > 5;

            if (5 < 10) {
               return true;
            } else {
               return false;
            }

            10 == 10;
            10 != 9;
        ";
        let expected = [
            Token {
                token_type: TokenType::LET,
                literal: "let".to_string(),
            },
            Token {
                token_type: TokenType::IDENTIFIER,
                literal: "five".to_string(),
            },
            Token {
                token_type: TokenType::ASSIGN,
                literal: "=".to_string(),
            },
            Token {
                token_type: TokenType::INT,
                literal: "5".to_string(),
            },
            Token {
                token_type: TokenType::SEMICOLON,
                literal: ";".to_string(),
            },
            Token {
                token_type: TokenType::LET,
                literal: "let".to_string(),
            },
            Token {
                token_type: TokenType::IDENTIFIER,
                literal: "ten".to_string(),
            },
            Token {
                token_type: TokenType::ASSIGN,
                literal: "=".to_string(),
            },
            Token {
                token_type: TokenType::INT,
                literal: "10".to_string(),
            },
            Token {
                token_type: TokenType::SEMICOLON,
                literal: ";".to_string(),
            },
            Token {
                token_type: TokenType::LET,
                literal: "let".to_string(),
            },
            Token {
                token_type: TokenType::IDENTIFIER,
                literal: "add".to_string(),
            },
            Token {
                token_type: TokenType::ASSIGN,
                literal: "=".to_string(),
            },
            Token {
                token_type: TokenType::FUNCTION,
                literal: "fn".to_string(),
            },
            Token {
                token_type: TokenType::LPAREN,
                literal: "(".to_string(),
            },
            Token {
                token_type: TokenType::IDENTIFIER,
                literal: "x".to_string(),
            },
            Token {
                token_type: TokenType::COMMA,
                literal: ",".to_string(),
            },
            Token {
                token_type: TokenType::IDENTIFIER,
                literal: "y".to_string(),
            },
            Token {
                token_type: TokenType::RPAREN,
                literal: ")".to_string(),
            },
            Token {
                token_type: TokenType::LBRACE,
                literal: "{".to_string(),
            },
            Token {
                token_type: TokenType::IDENTIFIER,
                literal: "x".to_string(),
            },
            Token {
                token_type: TokenType::PLUS,
                literal: "+".to_string(),
            },
            Token {
                token_type: TokenType::IDENTIFIER,
                literal: "y".to_string(),
            },
            Token {
                token_type: TokenType::SEMICOLON,
                literal: ";".to_string(),
            },
            Token {
                token_type: TokenType::RBRACE,
                literal: "}".to_string(),
            },
            Token {
                token_type: TokenType::SEMICOLON,
                literal: ";".to_string(),
            },
            Token {
                token_type: TokenType::LET,
                literal: "let".to_string(),
            },
            Token {
                token_type: TokenType::IDENTIFIER,
                literal: "result".to_string(),
            },
            Token {
                token_type: TokenType::ASSIGN,
                literal: "=".to_string(),
            },
            Token {
                token_type: TokenType::IDENTIFIER,
                literal: "add".to_string(),
            },
            Token {
                token_type: TokenType::LPAREN,
                literal: "(".to_string(),
            },
            Token {
                token_type: TokenType::IDENTIFIER,
                literal: "five".to_string(),
            },
            Token {
                token_type: TokenType::COMMA,
                literal: ",".to_string(),
            },
            Token {
                token_type: TokenType::IDENTIFIER,
                literal: "ten".to_string(),
            },
            Token {
                token_type: TokenType::RPAREN,
                literal: ")".to_string(),
            },
            Token {
                token_type: TokenType::SEMICOLON,
                literal: ";".to_string(),
            },
            Token {
                token_type: TokenType::BANG,
                literal: "!".to_string(),
            },
            Token {
                token_type: TokenType::MINUS,
                literal: "-".to_string(),
            },
            Token {
                token_type: TokenType::SLASH,
                literal: "/".to_string(),
            },
            Token {
                token_type: TokenType::ASTERISK,
                literal: "*".to_string(),
            },
            Token {
                token_type: TokenType::INT,
                literal: "5".to_string(),
            },
            Token {
                token_type: TokenType::SEMICOLON,
                literal: ";".to_string(),
            },
            Token {
                token_type: TokenType::INT,
                literal: "5".to_string(),
            },
            Token {
                token_type: TokenType::LT,
                literal: "<".to_string(),
            },
            Token {
                token_type: TokenType::INT,
                literal: "10".to_string(),
            },
            Token {
                token_type: TokenType::GT,
                literal: ">".to_string(),
            },
            Token {
                token_type: TokenType::INT,
                literal: "5".to_string(),
            },
            Token {
                token_type: TokenType::SEMICOLON,
                literal: ";".to_string(),
            },
            Token {
                token_type: TokenType::IF,
                literal: "if".to_string(),
            },
            Token {
                token_type: TokenType::LPAREN,
                literal: "(".to_string(),
            },
            Token {
                token_type: TokenType::INT,
                literal: "5".to_string(),
            },
            Token {
                token_type: TokenType::LT,
                literal: "<".to_string(),
            },
            Token {
                token_type: TokenType::INT,
                literal: "10".to_string(),
            },
            Token {
                token_type: TokenType::RPAREN,
                literal: ")".to_string(),
            },
            Token {
                token_type: TokenType::LBRACE,
                literal: "{".to_string(),
            },
            Token {
                token_type: TokenType::RETURN,
                literal: "return".to_string(),
            },
            Token {
                token_type: TokenType::TRUE,
                literal: "true".to_string(),
            },
            Token {
                token_type: TokenType::SEMICOLON,
                literal: ";".to_string(),
            },
            Token {
                token_type: TokenType::RBRACE,
                literal: "}".to_string(),
            },
            Token {
                token_type: TokenType::ELSE,
                literal: "else".to_string(),
            },
            Token {
                token_type: TokenType::LBRACE,
                literal: "{".to_string(),
            },
            Token {
                token_type: TokenType::RETURN,
                literal: "return".to_string(),
            },
            Token {
                token_type: TokenType::FALSE,
                literal: "false".to_string(),
            },
            Token {
                token_type: TokenType::SEMICOLON,
                literal: ";".to_string(),
            },
            Token {
                token_type: TokenType::RBRACE,
                literal: "}".to_string(),
            },
            Token {
                token_type: TokenType::INT,
                literal: "10".to_string(),
            },
            Token {
                token_type: TokenType::EQ,
                literal: "==".to_string(),
            },
            Token {
                token_type: TokenType::INT,
                literal: "10".to_string(),
            },
            Token {
                token_type: TokenType::SEMICOLON,
                literal: ";".to_string(),
            },
            Token {
                token_type: TokenType::INT,
                literal: "10".to_string(),
            },
            Token {
                token_type: TokenType::NEQ,
                literal: "!=".to_string(),
            },
            Token {
                token_type: TokenType::INT,
                literal: "9".to_string(),
            },
            Token {
                token_type: TokenType::SEMICOLON,
                literal: ";".to_string(),
            },
            Token {
                token_type: TokenType::EOF,
                literal: "EOF".to_string(),
            },
        ];

        let mut l = Lexer::new(input);

        for i in 0..expected.len() {
            let token = l.next_token();
            println!("{:?}", token);
            assert_eq!(token, expected[i]);
        }
    }
}
