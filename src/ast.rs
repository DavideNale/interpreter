use std::fmt;

use crate::token;

#[derive(Default)]
pub struct Ast {
    pub statements: Vec<Statement>,
}

impl Ast {
    pub fn new() -> Self {
        Ast::default()
    }
}

impl From<Vec<Statement>> for Ast {
    fn from(statements: Vec<Statement>) -> Self {
        Self { statements }
    }
}

impl fmt::Display for Ast {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.statements
                .iter()
                .map(|stmt| stmt.to_string())
                .collect::<Vec<String>>()
                .join(""),
        )
    }
}

pub enum Node {
    Statement(Statement),
    Expression,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!();
    }
}

#[derive(Clone)]
pub enum Statement {
    Let {
        token: token::Token,
        expr: Expression,
    },
    Return {
        token: token::Token,
        expr: Expression,
    },
    Expression {
        token: token::Token,
        expr: Expression,
    },
    Block {
        body: Vec<Statement>,
    },
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::Let { token, expr } => {
                write!(f, "let {} = {};", token.to_string(), expr.to_string())
            }
            Statement::Return { token, expr } => todo!(),
            Statement::Expression { token, expr } => todo!(),
            Statement::Block { body } => todo!(),
        }
    }
}

#[derive(Clone)]
pub enum Expression {
    Identifier(String),
    Boolean(bool),
    Int(String),
    PrefixExpr {
        op: token::Token,
        right: Box<Expression>,
    },
    InfixExpr {
        op: token::Token,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    If {
        condition: Box<Expression>,
        consequence: Box<Statement>,
        alternative: Option<Box<Statement>>,
    },
    Function {
        parameters: Vec<Expression>,
        body: Box<Statement>,
    },
    Call {
        function: Box<Expression>,
        arguments: Box<Expression>,
    },
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Int(s) => write!(f, "{}", s),
            _ => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ast_display() {
        let ast = Ast::from(vec![Statement::Let {
            expr: Expression::Int("5".to_string()),
            token: token::Token::Ident("x".to_string()),
        }]);

        assert_eq!(ast.to_string(), "let x = 5;");
    }
}
