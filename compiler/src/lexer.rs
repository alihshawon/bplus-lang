//compiler/src/lexer.rs


use crate::token::{lookup_ident, Token, TokenType};

pub struct Lexer {
    input: String,
    position: usize,      // current position in input
    read_position: usize, // current reading position (after current char)
    ch: u8,               // current char under examination
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut l = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: 0,
        };
        l.read_char();
        l
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0; // NUL character, signifies EOF
        } else {
            self.ch = self.input.as_bytes()[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn peek_char(&self) -> u8 {
        if self.read_position >= self.input.len() {
            0
        } else {
            self.input.as_bytes()[self.read_position]
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let tok = match self.ch {
            b'=' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    Token::new(TokenType::Eq, "==")
                } else {
                    Token::new(TokenType::Assign, "=")
                }
            }
            b';' => Token::new(TokenType::Semicolon, ";"),
            b'(' => Token::new(TokenType::LParen, "("),
            b')' => Token::new(TokenType::RParen, ")"),
            b',' => Token::new(TokenType::Comma, ","),
            b'+' => Token::new(TokenType::Plus, "+"),
            b'-' => Token::new(TokenType::Minus, "-"),
            b'!' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    Token::new(TokenType::NotEq, "!=")
                } else {
                    Token::new(TokenType::Bang, "!")
                }
            }
            b'/' => Token::new(TokenType::Slash, "/"),
            b'*' => Token::new(TokenType::Asterisk, "*"),
            b'<' => Token::new(TokenType::Lt, "<"),
            b'>' => Token::new(TokenType::Gt, ">"),
            b'{' => Token::new(TokenType::LBrace, "{"),
            b'}' => Token::new(TokenType::RBrace, "}"),
            b'"' => {
                let literal = self.read_string();
                return Token::new(TokenType::String, &literal);
            }
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                let literal = self.read_identifier();
                let token_type = lookup_ident(&literal);
                return Token::new(token_type, &literal);
            }
            b'0'..=b'9' => {
                let literal = self.read_number();
                return Token::new(TokenType::Int, &literal);
            }
            0 => Token::new(TokenType::Eof, ""),
            _ => Token::new(TokenType::Illegal, &(self.ch as char).to_string()),
        };

        self.read_char();
        tok
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while self.ch.is_ascii_alphabetic() || self.ch == b'_' {
            self.read_char();
        }
        self.input[position..self.position].to_string()
    }

    fn read_number(&mut self) -> String {
        let position = self.position;
        while self.ch.is_ascii_digit() {
            self.read_char();
        }
        self.input[position..self.position].to_string()
    }

    fn read_string(&mut self) -> String {
        let position = self.position + 1;
        loop {
            self.read_char();
            if self.ch == b'"' || self.ch == 0 {
                break;
            }
        }
        self.input[position..self.position].to_string()
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }
}
