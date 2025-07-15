// compiler/src/perser.rs

use crate::lexer::{Lexer, Token, Position};

#[derive(Debug)]
pub enum Expr {
    Number(i64, Position),
    Variable(String, Position),
    Binary {
        left: Box<Expr>,
        op: String,
        right: Box<Expr>,
        pos: Position,
    },
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(lexer: &mut Lexer) -> Self {
        let tokens = lexer.tokenize();
        Parser {
            tokens,
            current: 0,
        }
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.current).unwrap_or(&Token::EOF(Position { line: 0, column: 0 }))
    }

    fn advance(&mut self) {
        if self.current < self.tokens.len() {
            self.current += 1;
        }
    }

    fn match_token(&mut self, expected: fn(&Token) -> bool) -> Option<Token> {
        let token = self.peek();
        if expected(token) {
            let tok = token.clone();
            self.advance();
            Some(tok)
        } else {
            None
        }
    }

    pub fn parse_expression(&mut self) -> Option<Expr> {
        self.parse_term()
    }

    fn parse_term(&mut self) -> Option<Expr> {
        let mut expr = self.parse_factor()?;

        while let Some(op_token) = self.match_token(|t| matches!(t, Token::Plus(_) | Token::Minus(_))) {
            let op_str = match &op_token {
                Token::Plus(_) => "+",
                Token::Minus(_) => "-",
                _ => unreachable!(),
            }
            .to_string();
            let pos = get_pos(&op_token);
            let right = self.parse_factor()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op: op_str,
                right: Box::new(right),
                pos,
            };
        }

        Some(expr)
    }

    fn parse_factor(&mut self) -> Option<Expr> {
        let mut expr = self.parse_primary()?;

        while let Some(op_token) = self.match_token(|t| matches!(t, Token::Star(_) | Token::Slash(_))) {
            let op_str = match &op_token {
                Token::Star(_) => "*",
                Token::Slash(_) => "/",
                _ => unreachable!(),
            }
            .to_string();
            let pos = get_pos(&op_token);
            let right = self.parse_primary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op: op_str,
                right: Box::new(right),
                pos,
            };
        }

        Some(expr)
    }

    fn parse_primary(&mut self) -> Option<Expr> {
        match self.peek() {
            Token::Number(value, pos) => {
                let expr = Expr::Number(*value, pos.clone());
                self.advance();
                Some(expr)
            }
            Token::Identifier(name, pos) => {
                let expr = Expr::Variable(name.clone(), pos.clone());
                self.advance();
                Some(expr)
            }
            Token::LParen(_) => {
                self.advance();
                let expr = self.parse_expression()?;
                if self.match_token(|t| matches!(t, Token::RParen(_))).is_none() {
                    eprintln!("Expected ')' after expression");
                    return None;
                }
                Some(expr)
            }
            tok => {
                eprintln!("Unexpected token in expression: {:?}", tok);
                None
            }
        }
    }
}

fn get_pos(token: &Token) -> Position {
    match token {
        Token::Number(_, pos)
        | Token::Identifier(_, pos)
        | Token::Plus(pos)
        | Token::Minus(pos)
        | Token::Star(pos)
        | Token::Slash(pos)
        | Token::LParen(pos)
        | Token::RParen(pos)
        | Token::EOF(pos)
        | Token::Unknown(_, pos) => pos.clone(),
    }
}