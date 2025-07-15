//compiler/src/lexer.rs

use crate::token::{Token, TokenType};

pub struct Lexer {
    input: String,
    position: usize,      // current position in input (points to current char)
    read_position: usize, // current reading position in input (after current char)
    ch: Option<char>,     // current char under examination
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut l = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: None,
        };
        l.read_char();
        l
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

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let tok = match self.ch {
            Some('=') => {
                if self.peek_char() == Some('=') {
                    self.read_char();
                    Token::new(TokenType::EQ, "==")
                } else {
                    Token::new(TokenType::ASSIGN, "=")
                }
            }
            Some(';') => Token::new(TokenType::SEMICOLON, ";"),
            Some('(') => Token::new(TokenType::LPAREN, "("),
            Some(')') => Token::new(TokenType::RPAREN, ")"),
            Some(',') => Token::new(TokenType::COMMA, ","),
            Some('+') => Token::new(TokenType::PLUS, "+"),
            Some('-') => Token::new(TokenType::MINUS, "-"),
            Some('!') => {
                if self.peek_char() == Some('=') {
                    self.read_char();
                    Token::new(TokenType::NOT_EQ, "!=")
                } else {
                    Token::new(TokenType::BANG, "!")
                }
            }
            Some('/') => Token::new(TokenType::SLASH, "/"),
            Some('*') => Token::new(TokenType::ASTERISK, "*"),
            Some('<') => Token::new(TokenType::LT, "<"),
            Some('>') => Token::new(TokenType::GT, ">"),
            Some(ch) if is_letter(ch) => {
                let literal = self.read_identifier();
                let token_type = TokenType::lookup_ident(&literal);
                return Token::new(token_type, &literal);
            }
            Some(ch) if ch.is_ascii_digit() => {
                let literal = self.read_number();
                return Token::new(TokenType::INT, &literal);
            }
            None => Token::new(TokenType::EOF, ""),
            Some(_) => Token::new(TokenType::ILLEGAL, ""),
        };

        self.read_char();
        tok
    }

    fn peek_char(&self) -> Option<char> {
        if self.read_position >= self.input.len() {
            None
        } else {
            self.input.chars().nth(self.read_position)
        }
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while let Some(ch) = self.ch {
            if !is_letter(ch) {
                break;
            }
            self.read_char();
        }
        self.input[position..self.position].to_string()
    }

    fn read_number(&mut self) -> String {
        let position = self.position;
        while let Some(ch) = self.ch {
            if !ch.is_ascii_digit() {
                break;
            }
            self.read_char();
        }
        self.input[position..self.position].to_string()
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.ch {
            if ch.is_whitespace() {
                self.read_char();
            } else {
                break;
            }
        }
    }
}

fn is_letter(ch: char) -> bool {
    ch.is_ascii_alphabetic() || ch == '_'
}