// compiler/src/lexer.rs

use crate::token::{lookup_ident, Token, TokenType};

pub struct Lexer {
    input: String,
    position: usize,      // current position in input (points to current char)
    read_position: usize, // next position to read (after current char)
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
            self.ch = 0; // NUL character to signify EOF
        } else {
            // Check if current position points to a Bengali unicode letter (3 bytes)
            if self.is_unicode_bengali_letter() {
                // We will read only the first byte here for lexer 'ch'
                // But actual identifier reading reads the full string
                self.ch = self.input.as_bytes()[self.position];
            } else {
                self.ch = self.input.as_bytes()[self.read_position];
            }
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
            // Identifiers: ASCII letters, underscore, or Bengali unicode letters
            _ if self.ch.is_ascii_alphabetic() || self.ch == b'_' || self.is_unicode_bengali_letter() => {
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
        let start_pos = self.position;

        while self.ch.is_ascii_alphabetic() || self.ch == b'_' || self.is_unicode_bengali_letter() {
            self.read_char();
        }

        self.input[start_pos..self.position].to_string()
    }

    fn read_number(&mut self) -> String {
        let start_pos = self.position;

        while self.ch.is_ascii_digit() {
            self.read_char();
        }

        self.input[start_pos..self.position].to_string()
    }

    fn read_string(&mut self) -> String {
        let start_pos = self.position + 1; // skip opening quote

        loop {
            self.read_char();
            if self.ch == b'"' || self.ch == 0 {
                break;
            }
        }

        let result = self.input[start_pos..self.position].to_string();

        self.read_char(); // skip closing quote

        result
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }

    /// Check if current character is start of a Bengali Unicode letter (3-byte UTF-8 sequence)
    /// Bengali Unicode range: U+0980 to U+09FF
    fn is_unicode_bengali_letter(&self) -> bool {
        // Check if enough bytes available
        if self.position + 3 > self.input.len() {
            return false;
        }
        let bytes = &self.input.as_bytes()[self.position..self.position + 3];
        // UTF-8 Bengali chars start with 0xE0, second byte 0xA6-0xAF, third byte 0x80-0xBF
        bytes[0] == 0xE0 && (0xA6..=0xAF).contains(&bytes[1]) && (0x80..=0xBF).contains(&bytes[2])
    }
}
