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
            self.ch = 0; // EOF
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

        // Check for comments first (single-line or multi-line)
        if self.ch == b'/' {
            if self.peek_char() == b'/' {
                self.read_char();
                self.read_char();
                self.skip_single_line_comment();
                return self.next_token();
            } else if self.peek_char() == b'*' {
                self.read_char();
                self.read_char();
                if let Err(err) = self.skip_multi_line_comment("/*", "*/") {
                    return Token::new(TokenType::Illegal, &err);
                }
                return self.next_token();
            }
        } else if self.ch == b'#' {
            // Python, Ruby, Perl, Bash style single line comment
            self.read_char();
            self.skip_single_line_comment();
            return self.next_token();
        } else if self.ch == b'-' && self.peek_char() == b'-' {
            // SQL, Haskell style single line comment --
            self.read_char();
            self.read_char();
            self.skip_single_line_comment();
            return self.next_token();
        } else if self.ch == b'=' {
            // Ruby multiline comment =begin ... =end
            let lookahead = self.peek_n_chars(5);
            if lookahead == "begin" {
                // consume =begin
                for _ in 0..6 { self.read_char(); } // '=' + 'begin'
                if let Err(err) = self.skip_multi_line_comment("=begin", "=end") {
                    return Token::new(TokenType::Illegal, &err);
                }
                return self.next_token();
            }
        } else if self.ch == b'{' && self.peek_char() == b'-' {
            // Haskell multiline comment {- ... -}
            self.read_char();
            self.read_char();
            if let Err(err) = self.skip_multi_line_comment("{-", "-}") {
                return Token::new(TokenType::Illegal, &err);
            }
            return self.next_token();
        } else if self.ch == b'(' && self.peek_char() == b'*' {
            // OCaml, Pascal multiline comment (* ... *)
            self.read_char();
            self.read_char();
            if let Err(err) = self.skip_multi_line_comment("(*", "*)") {
                return Token::new(TokenType::Illegal, &err);
            }
            return self.next_token();
        } else if self.ch == b'"' {
            // Check for Python multiline docstring style """ ... """
            let lookahead = self.peek_n_chars(2);
            if lookahead == "\"\"" {
                // consume opening """
                self.read_char();
                self.read_char();
                self.read_char();
                if let Err(err) = self.skip_multi_line_comment("\"\"\"", "\"\"\"") {
                    return Token::new(TokenType::Illegal, &err);
                }
                return self.next_token();
            }
        } else if self.ch == b'\'' {
            // Check for Python multiline docstring style ''' ... '''
            let lookahead = self.peek_n_chars(2);
            if lookahead == "''" {
                // consume opening '''
                self.read_char();
                self.read_char();
                self.read_char();
                if let Err(err) = self.skip_multi_line_comment("'''", "'''") {
                    return Token::new(TokenType::Illegal, &err);
                }
                return self.next_token();
            }
        }

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
            b'.' => Token::new(TokenType::Fullstop, "."), // <-- Added line for fullstop token

            // Identifiers: ASCII letters, underscore, or Bengali unicode letters
 _ if self.ch.is_ascii_alphabetic() || self.ch == b'_' || self.is_unicode_bengali_letter() => {
    let first_literal = self.read_identifier();

    if first_literal == "input" {
        self.skip_whitespace();

        // Save lexer state to rewind if next word is not "nao"
        let saved_position = self.position;
        let saved_read_position = self.read_position;
        let saved_ch = self.ch;

        if self.ch.is_ascii_alphabetic() || self.ch == b'_' || self.is_unicode_bengali_letter() {
            let second_literal = self.read_identifier();

            if second_literal == "nao" {
                return Token::new(TokenType::InputNao, "input nao");
            } else {
                // rewind lexer state if not matched
                self.position = saved_position;
                self.read_position = saved_read_position;
                self.ch = saved_ch;
            }
        }
    }

    let token_type = lookup_ident(&first_literal);
    return Token::new(token_type, &first_literal);
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

    fn skip_single_line_comment(&mut self) {
        while self.ch != b'\n' && self.ch != 0 {
            self.read_char();
        }
    }

    fn skip_multi_line_comment(&mut self, start: &str, end: &str) -> Result<(), String> {
        let mut end_matched = 0;
        let end_bytes = end.as_bytes();
        let end_len = end_bytes.len();

        loop {
            if self.ch == 0 {
                return Err(format!("Unterminated multi-line comment starting with {}", start));
            }
            if self.ch == end_bytes[end_matched] {
                end_matched += 1;
                if end_matched == end_len {
                    self.read_char(); // consume last char of end delimiter
                    break;
                }
            } else {
                end_matched = 0;
            }
            self.read_char();
        }
        Ok(())
    }

    fn peek_n_chars(&self, n: usize) -> String {
        let mut result = String::new();
        let start = self.position + 1;
        let end = (start + n).min(self.input.len());

        if start >= self.input.len() {
            return result;
        }

        for i in start..end {
            result.push(self.input.as_bytes()[i] as char);
        }
        result
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

