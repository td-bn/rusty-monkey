use std::fmt::{Display, Formatter, Write};

pub type Identifier = String;

pub enum Statement {
    Let(Identifier, Expr),
    Return(Expr),
}
impl Display for Statement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::Let(id, expr) => f.write_str(&*format!("let {} = {}", id, expr)),
            Statement::Return(expr) => f.write_str(&*format!("return {}", expr)),
        }
    }
}

pub enum Expr {
    Identifier(Identifier),
}
impl Display for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Identifier(id) => f.write_str(&*id.to_string()),
        }
    }
}

pub struct Program {
    pub statements: Vec<Statement>,
}

impl Program {
    fn token_literal(&self) -> String {
        match self.statements.len() {
            0 => "".to_string(),
            _ => self.statements.first().unwrap().to_string(),
        }
    }
}
