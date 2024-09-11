use crate::ast::{self, Expression};
use crate::lexer::Lexer;
use crate::token::Token;

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum Precedence {
    Lowest,
    Equals,
    LessGreater,
    Sum,
    Product,
    Prefix,
    Call,
}

pub fn get_token_precedence(token: &Token) -> Precedence {
    match token {
        Token::Eq | Token::NotEq => Precedence::Equals,
        Token::Lt | Token::Gt => Precedence::LessGreater,
        Token::Plus | Token::Minus => Precedence::Sum,
        Token::Slash | Token::Asterisk => Precedence::Product,
        Token::LParen => Precedence::Call,
        _ => Precedence::Lowest,
    }
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
            println!("{}", tok.to_string());
            let stmt = match tok {
                Token::Let => self.parse_let(),
                Token::Return => self.parse_return(),
                _ => self.parse_expression_statement(),
            };
            if let Some(parsed_stmt) = stmt {
                ast.statements.push(parsed_stmt);
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
        let expr = self.parse_expression(Precedence::Lowest)?;
        let stmt = ast::Statement::Let {
            token: Token::Let,
            expr,
        };
        let peek = self.lexer.peek()?;
        if matches!(peek, Token::Semicolon) {
            self.lexer.next();
        }
        Some(stmt)
    }

    fn parse_return(&mut self) -> Option<ast::Statement> {
        self.lexer.next();
        let peek = self.lexer.peek()?;
        let expr = self.parse_expression(Precedence::Lowest)?;
        let stmt = ast::Statement::Return {
            token: peek.clone(),
            expr,
        };
        if matches!(peek, Token::Semicolon) {
            self.lexer.next()?;
        }
        Some(stmt)
    }

    fn parse_expression_statement(&mut self) -> Option<ast::Statement> {
        let peek = self.lexer.peek()?;
        let expr = self.parse_expression(Precedence::Lowest)?;
        if matches!(peek, Token::Semicolon) {
            self.lexer.next()?;
        }
        Some(ast::Statement::Expression { token: peek, expr })
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Option<ast::Expression> {
        let mut lhs = self.parse_prefix()?;
        self.lexer.next();
        let peek = self.lexer.peek()?;

        while !matches!(peek, Token::Semicolon) && precedence < get_token_precedence(&peek) {
            self.lexer.next();
            lhs = self.parse_infix(&lhs)?;
        }

        Some(lhs)
    }

    fn parse_block(&mut self) -> Option<ast::Statement> {
        todo!()
    }

    fn parse_prefix(&mut self) -> Option<ast::Expression> {
        let peek = self.lexer.peek()?;
        match peek {
            Token::Ident(i) => Some(Expression::Identifier(i)),
            Token::Int(i) => Some(Expression::Int(i)),
            Token::Bang | Token::Minus => {
                let next = self.lexer.next()?;
                Some(ast::Expression::PrefixExpr {
                    op: next.clone(),
                    right: Box::new(self.parse_expression(Precedence::Prefix).unwrap()),
                })
            }
            Token::True | Token::False => {
                let value = matches!(peek, Token::True);
                Some(Expression::Boolean(value))
            }
            Token::LParen => {
                let next = self.lexer.next()?;
                let expr = self.parse_expression(Precedence::Lowest)?;
                let peek = self.lexer.peek()?;
                if matches!(peek, Token::RParen) {
                    return None;
                }
                return Some(expr);
            }
            Token::If => {
                let mut peek = self.lexer.peek()?;
                if !matches!(peek, Token::LParen) {
                    return None;
                }
                self.lexer.next();

                let condition = self.parse_expression(Precedence::Lowest)?;
                peek = self.lexer.peek()?;
                if !matches!(peek, Token::RParen) {
                    return None;
                }
                self.lexer.next();
                peek = self.lexer.peek()?;
                if !matches!(peek, Token::LBrace) {
                    return None;
                }
                self.lexer.next();
                let consequence = self.parse_block()?;
                peek = self.lexer.peek()?;
                if matches!(peek, Token::Else) {
                    self.lexer.next()?;
                    peek = self.lexer.peek()?;
                    if !matches!(peek, Token::LBrace) {
                        return None;
                    }
                    self.lexer.next();
                }
                Some(ast::Expression::If {
                    condition: Box::new(condition),
                    consequence: Box::new(consequence),
                    alternative: Some(Box::new(self.parse_block()?)),
                })
            }
            Token::Function => todo!(),
            _ => panic!("no prefix parsing funciton"),
        }
    }

    fn parse_infix(&mut self, left: &Expression) -> Option<ast::Expression> {
        let peek = self.lexer.peek()?;
        match peek {
            Token::Plus
            | Token::Minus
            | Token::Slash
            | Token::Asterisk
            | Token::Eq
            | Token::NotEq
            | Token::Lt
            | Token::Gt => {
                let curr = self.lexer.next()?;
                let precedence = get_token_precedence(&curr);
                self.lexer.next()?;
                let right = self.parse_expression(precedence)?;
                Some(ast::Expression::InfixExpr {
                    op: curr.clone(),
                    left: Box::new(left.clone()),
                    right: Box::new(right.clone()),
                })
            }
            _ => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_let() {
        let s = "let x = 5;";
        Parser::new(s.to_string()).parse_program();
    }
}
