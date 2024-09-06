use crate::ast;
use crate::lexer::Lexer;
use crate::token::Token;

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum Precedence {
    Lowest,
    Equals,
    LessGreater,
    Sum,
    Product,
    Call,
    Index,
}

pub struct Parser {
    lexer: Lexer,
}

impl Parser {
    pub fn new(input: String) -> Parser {
        Parser {
            lexer: Lexer::new(input),
        }
    }

    pub fn parse_program(&mut self) -> ast::Ast {
        let mut ast = ast::Ast::new();

        while let Some(tok) = self.lexer.next() {
            if matches!(tok, Token::Eof) {
                break;
            }
            let stmt = match tok {
                Token::Let => self.parse_let(),
                Token::Return => self.parse_return(),
                _ => self.parse_expression_statement(),
            };
            if let Some(parsed_stmt) = stmt {
                ast.statements.push(parsed_stmt)
            }
        }

        ast
    }

    fn parse_let(&mut self) -> Option<ast::Statement> {
        let peek = self.lexer.peek()?;
        if !matches!(peek, Token::Ident(_)) {
            return None;
        }
        self.lexer.next();
        let peek = self.lexer.peek()?;
        if !matches!(peek, Token::Assign) {
            return None;
        }
        self.lexer.next();
        let expr = self.parse_expression()?;
        let stmt = ast::Statement::Let {
            token: Token::Let,
            expr,
        };
        Some(stmt)
    }

    fn parse_return(&mut self) -> Option<ast::Statement> {
        todo!();
    }

    fn parse_expression_statement(&mut self) -> Option<ast::Statement> {
        todo!()
    }

    fn parse_expression(&mut self) -> Option<ast::Expression> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_let() {
        let s = "let x = 5;";
        // Parser::new(s.to_string()).parse_program()
    }
}
