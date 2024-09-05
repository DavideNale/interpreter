use std::fmt;

use crate::token;

pub struct Ast {
    statements: Vec<Statement>,
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
            Statement::Let { token, expr } => todo!(),
            Statement::Return { token, expr } => todo!(),
            Statement::Expression { token, expr } => todo!(),
            Statement::Block { body } => todo!(),
        }
    }
}

pub enum Expression {
    // Identifier(Ident),
    // Boolean(Bool),
    // Prefix(),
    // Infix(),
    // If(),
    // Function(),
    // Call(),
}
