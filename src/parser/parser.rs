use crate::lexer::Lexer;
use crate::parser::ast::{Expr, Program, Statement};
use crate::token::token::{Token, TokenType};

pub struct Parser {
    lexer: Lexer,
    curr_token: Box<Token>,
    peek_token: Box<Token>,
    errors: Vec<String>,
}

impl Parser {
    pub fn new(l: Lexer) -> Self {
        let mut p = Self {
            lexer: l,
            curr_token: Box::new(Token::default()),
            peek_token: Box::new(Token::default()),
            errors: Vec::new(),
        };
        p.next_token();
        p.next_token();
        p
    }

    pub fn next_token(&mut self) {
        // TODO: Note this in Notion
        self.curr_token =
            std::mem::replace(&mut self.peek_token, Box::new(self.lexer.next_token()));
    }

    pub fn parse_program(&mut self) -> Program {
        let mut statements = Vec::new();
        while !self.current_token_is(TokenType::EOF) {
            if let Some(stmt) = self.parse_statement() {
                statements.push(stmt)
            }
            self.next_token();
        }
        Program { statements }
    }

    pub fn parse_statement(&mut self) -> Option<Statement> {
        match self.curr_token.token_type {
            TokenType::LET => self.parse_let_statement(),
            TokenType::RETURN => self.parse_return_statement(),
            _ => None,
        }
    }

    pub fn parse_let_statement(&mut self) -> Option<Statement> {
        if !self.expect_peek_token(TokenType::IDENTIFIER) {
            return None;
        }
        let literal = &self.curr_token.literal.clone();
        if !self.expect_peek_token(TokenType::ASSIGN) {
            return None;
        }
        // TODO: Skipping till semicolon
        while !self.current_token_is(TokenType::SEMICOLON) {
            self.next_token();
        }
        Some(Statement::Let(
            literal.to_string(),
            Expr::Identifier("dummy".to_string()),
        ))
    }

    pub fn parse_return_statement(&mut self) -> Option<Statement> {
        self.next_token();
        // TODO: Skipping till semicolon
        while !self.current_token_is(TokenType::SEMICOLON) {
            self.next_token();
        }
        Some(Statement::Return(Expr::Identifier("dummy".to_string())))
    }

    pub fn errors(&self) -> &Vec<String> {
        self.errors.as_ref()
    }

    fn peek_error(&mut self, t: TokenType) {
        let error = format!(
            "Expected next token to be {:?} got {:?} instead.",
            t, self.peek_token.token_type
        );
        self.errors.push(error)
    }

    fn current_token_is(&mut self, t: TokenType) -> bool {
        self.curr_token.token_type == t
    }

    fn peek_token_is(&mut self, t: TokenType) -> bool {
        self.peek_token.token_type == t
    }

    fn expect_peek_token(&mut self, t: TokenType) -> bool {
        match self.peek_token_is(t) {
            true => {
                self.next_token();
                true
            }
            _ => {
                self.peek_error(t);
                false
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast::Statement::Let;
    use crate::parser::ast::{Expr, Identifier};

    #[test]
    fn test_let_statements() {
        let input = "\
        let x = 5;\
        let foo = 10;";
        let l = Lexer::new(input);
        let mut parser = Parser::new(l);

        let program = parser.parse_program();
        check_parser_errors(&parser);
        if program.statements.is_empty() {
            panic!("Parse program has no statements")
        }
        let expected_identifiers = ["x".to_string(), "foo".to_string()];
        if program.statements.len() != expected_identifiers.len() {
            panic!(
                "Expected {} statements, got {}",
                expected_identifiers.len(),
                program.statements.len()
            )
        }

        for i in 0..expected_identifiers.len() {
            let statement = program.statements.get(i).unwrap();
            match statement {
                Let(id, expr) => test_let_statement(id, expr, &expected_identifiers[i]),
                _ => panic!("Not a LET statement"),
            }
        }
    }

    #[test]
    fn test_return_statements() {
        let input = "\
        return 5;\
        return 29;";
        let l = Lexer::new(input);
        let mut parser = Parser::new(l);

        let program = parser.parse_program();
        check_parser_errors(&parser);

        if program.statements.len() != 2 {
            panic!("Parse program doesn't have 2 statements")
        }
    }

    fn test_let_statement(id: &Identifier, expr: &Expr, expected: &String) {
        assert_eq!(id.to_string(), *expected);
    }

    fn check_parser_errors(parser: &Parser) {
        let errors = parser.errors();
        if errors.is_empty() {
            return;
        }
        println!("Parsing failed with {} errors", errors.len());
        for e in errors {
            println!("{}", e);
        }
    }
}
