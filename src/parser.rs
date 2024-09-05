use crate::ast::Ast;
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

    pub fn parse_program(&mut self) -> ast::Ast{
        let mut ast = ast::Ast::new();

        while let Some(tok) = self.lexer.next() {
            match tok {
                Token::Eof => return ast,
                Token::Let => self.parse_statement(),
                _ => {
                    println!("{}\n", tok)
                }
            }
        }
    }

    fn parse_statement(self) -> Option<ast::Statement> {
       if let Some(peek) = self.lexer.peek() {
           if peek != Token::Ident{
               None
           }
           if peer != Token::Assign {
               None
           }

           for let while Some(next) = self.lexer.next(), next != Token::Semicolon{
           //
           }
           return Some(Statement::new())
       }
    }

    fn expect_peek() -> bool {
        todo!();
    }

    fn parse_expression(self) {
        todo!();
    }

    
    // func (p *Parser) parseLetStatement() *ast.LetStatement {
    // stmt := &ast.LetStatement{Token: p.curToken}
    // if !p.expectPeek(token.IDENT) {
    // return nil
    // }
    // stmt.Name = &ast.Identifier{Token: p.curToken, Value: p.curToken.Literal}
    // if !p.expectPeek(token.ASSIGN) {
    // return nil
    // }
    // // TODO: We're skipping the expressions until we
    // // encounter a semicolon
    // for !p.curTokenIs(token.SEMICOLON) {
    // p.nextToken()
    // }
    // return stmt
    // }
    // func (p *Parser) curTokenIs(t token.TokenType) bool {
    // return p.curToken.Type == t
    // }
    // func (p *Parser) peekTokenIs(t token.TokenType) bool {
    // return p.peekToken.Type == t
    // }
    // func (p *Parser) expectPeek(t token.TokenType) bool {
    // if p.peekTokenIs(t) {
    // p.nextToken()
    // return true
    // } else {
    // return false
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_let() {
        let s = "let x = 5;";
        Parser::new(s.to_string()).parse_program()
    }
}
