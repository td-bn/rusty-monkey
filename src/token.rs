use std::str::FromStr;
use strum_macros::EnumString;

#[derive(Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

#[derive(Debug, PartialEq, EnumString)]
pub enum TokenType {
    ILLEGAL,
    EOF,

    IDENTIFIER,
    INT,

    // KEYWORDS
    #[strum(serialize = "fn")]
    FUNCTION,
    #[strum(serialize = "let")]
    LET,
    #[strum(serialize = "if")]
    IF,
    #[strum(serialize = "else")]
    ELSE,
    #[strum(serialize = "true")]
    TRUE,
    #[strum(serialize = "false")]
    FALSE,
    #[strum(serialize = "return")]
    RETURN,

    // OPERATORS
    #[strum(serialize = "=")]
    ASSIGN,
    #[strum(serialize = "+")]
    PLUS,
    #[strum(serialize = "-")]
    MINUS,
    #[strum(serialize = "!")]
    BANG,
    #[strum(serialize = "*")]
    ASTERISK,
    #[strum(serialize = "/")]
    SLASH,
    #[strum(serialize = "<")]
    LT,
    #[strum(serialize = ">")]
    GT,
    #[strum(serialize = "==")]
    EQ,
    #[strum(serialize = "!=")]
    NEQ,

    #[strum(serialize = "(")]
    LPAREN,
    #[strum(serialize = ")")]
    RPAREN,
    #[strum(serialize = "{")]
    LBRACE,
    #[strum(serialize = "}")]
    RBRACE,
    #[strum(serialize = ";")]
    SEMICOLON,
    #[strum(serialize = ",")]
    COMMA,
}

impl TokenType {
    pub fn lookup_identifier(literal: &str) -> TokenType {
        match TokenType::from_str(literal) {
            Ok(token_type) => token_type,
            _ => TokenType::IDENTIFIER,
        }
    }
}
