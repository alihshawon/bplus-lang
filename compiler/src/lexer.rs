use crate::token::{lookup_ident, Token, TokenType};

pub struct Lexer {
    input: String,
    position: usize,      // Current index in input string (points to current char)
    read_position: usize, // Next index to read from input (after current char)
    ch: u8,               // Current byte (character) under examination
    line: usize,          // Current line number
    column: usize,        // Current column number
    token_start_line: usize,   // Track token start position (line)
    token_start_column: usize, // Track token start position (column)
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut l = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: 0,
            line: 1,
            column: 0,
            token_start_line: 1,
            token_start_column: 0,
        };
        l.read_char(); // Initialize first char
        l
    }

    fn read_char_literal(&mut self) -> Result<String, String> {
        // Assumes current char is starting `'`
        self.read_char(); // consume opening '

        let mut char_literal = String::new();

        if self.ch == b'\\' {
            // Escape sequence
            self.read_char();
            let escaped_char = match self.ch {
                b'n' => '\n',
                b't' => '\t',
                b'r' => '\r',
                b'\'' => '\'',
                b'\\' => '\\',
                other => other as char,
            };
            char_literal.push(escaped_char);
            self.read_char();
        } else if self.ch != 0 && self.ch != b'\'' {
            char_literal.push(self.ch as char);
            self.read_char();
        } else {
            return Err("Empty or invalid char literal".to_string());
        }

        if self.ch == b'\'' {
            self.read_char(); // consume closing '
            Ok(char_literal)
        } else {
            Err("Unterminated char literal".to_string())
        }
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0; // EOF
        } else {
            self.ch = self.input.as_bytes()[self.read_position];
        }

        self.position = self.read_position;
        self.read_position += 1;

        if self.ch == b'\n' {
            self.line += 1;
            self.column = 0;
        } else {
            self.column += 1;
        }
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

        // Mark token start position before reading token
        self.token_start_line = self.line;
        self.token_start_column = self.column;

        // Comment handling (same as before)
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
                    return Token::new(TokenType::Illegal, &err, self.token_start_line, self.token_start_column);
                }
                return self.next_token();
            }
        } else if self.ch == b'#' {
            self.read_char();
            self.skip_single_line_comment();
            return self.next_token();
        } else if self.ch == b'-' && self.peek_char() == b'-' {
            self.read_char();
            self.read_char();
            self.skip_single_line_comment();
            return self.next_token();
        } else if self.ch == b'=' {
            let lookahead = self.peek_n_chars(5);
            if lookahead == "begin" {
                for _ in 0..6 { self.read_char(); }
                if let Err(err) = self.skip_multi_line_comment("=begin", "=end") {
                    return Token::new(TokenType::Illegal, &err, self.token_start_line, self.token_start_column);
                }
                return self.next_token();
            }
        } else if self.ch == b'{' && self.peek_char() == b'-' {
            self.read_char();
            self.read_char();
            if let Err(err) = self.skip_multi_line_comment("{-", "-}") {
                return Token::new(TokenType::Illegal, &err, self.token_start_line, self.token_start_column);
            }
            return self.next_token();
        } else if self.ch == b'(' && self.peek_char() == b'*' {
            self.read_char();
            self.read_char();
            if let Err(err) = self.skip_multi_line_comment("(*", "*)") {
                return Token::new(TokenType::Illegal, &err, self.token_start_line, self.token_start_column);
            }
            return self.next_token();
        } else if self.ch == b'"' {
            let lookahead = self.peek_n_chars(2);
            if lookahead == "\"\"" {
                self.read_char();
                self.read_char();
                self.read_char();
                if let Err(err) = self.skip_multi_line_comment("\"\"\"", "\"\"\"") {
                    return Token::new(TokenType::Illegal, &err, self.token_start_line, self.token_start_column);
                }
                return self.next_token();
            }
        } else if self.ch == b'\'' {
            let lookahead = self.peek_n_chars(2);
            if lookahead == "''" {
                self.read_char();
                self.read_char();
                self.read_char();
                if let Err(err) = self.skip_multi_line_comment("'''", "'''") {
                    return Token::new(TokenType::Illegal, &err, self.token_start_line, self.token_start_column);
                }
                return self.next_token();
            }
        }

        let tok = match self.ch {
            b'=' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    Token::new(TokenType::Eq, "==", self.token_start_line, self.token_start_column)
                } else {
                    Token::new(TokenType::Assign, "=", self.token_start_line, self.token_start_column)
                }
            }
            b';' => Token::new(TokenType::Semicolon, ";", self.token_start_line, self.token_start_column),
            b'(' => Token::new(TokenType::LParen, "(", self.token_start_line, self.token_start_column),
            b')' => Token::new(TokenType::RParen, ")", self.token_start_line, self.token_start_column),
            b',' => Token::new(TokenType::Comma, ",", self.token_start_line, self.token_start_column),
            b'+' => Token::new(TokenType::Plus, "+", self.token_start_line, self.token_start_column),
            b'-' => Token::new(TokenType::Minus, "-", self.token_start_line, self.token_start_column),
            b'!' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    Token::new(TokenType::NotEq, "!=", self.token_start_line, self.token_start_column)
                } else {
                    Token::new(TokenType::Bang, "!", self.token_start_line, self.token_start_column)
                }
            }
            b'/' => Token::new(TokenType::Slash, "/", self.token_start_line, self.token_start_column),
            b'*' => Token::new(TokenType::Asterisk, "*", self.token_start_line, self.token_start_column),

            b'\'' => {
                match self.read_char_literal() {
                    Ok(lit) => return Token::new(TokenType::Char, &lit, self.token_start_line, self.token_start_column),
                    Err(e) => return Token::new(TokenType::Illegal, &e, self.token_start_line, self.token_start_column),
                }
            }

            b'<' => {
                if self.peek_char() == b'<' {
                    self.read_char();
                    Token::new(TokenType::ShiftLeft, "<<", self.token_start_line, self.token_start_column)
                } else {
                    Token::new(TokenType::Lt, "<", self.token_start_line, self.token_start_column)
                }
            }
            b'>' => {
                if self.peek_char() == b'>' {
                    self.read_char();
                    Token::new(TokenType::ShiftRight, ">>", self.token_start_line, self.token_start_column)
                } else {
                    Token::new(TokenType::Gt, ">", self.token_start_line, self.token_start_column)
                }
            }
            b'{' => Token::new(TokenType::LBrace, "{", self.token_start_line, self.token_start_column),
            b'}' => Token::new(TokenType::RBrace, "}", self.token_start_line, self.token_start_column),
            b'"' => {
                match self.read_string() {
                    Ok(lit) => return Token::new(TokenType::String, &lit, self.token_start_line, self.token_start_column),
                    Err(e) => return Token::new(TokenType::Illegal, &e, self.token_start_line, self.token_start_column),
                }
            }
            b'.' => Token::new(TokenType::Fullstop, ".", self.token_start_line, self.token_start_column),

_ if self.ch.is_ascii_alphabetic() || self.ch == b'_' || self.is_unicode_bengali_letter() => {
    // প্রথম word পড়া
    let first_word = self.read_identifier();
    let mut literal = first_word.clone();
    let mut token_type = lookup_ident(&literal);

    // multi-word keywords handle করার জন্য loop
    loop {
        let saved_pos = self.position;
        let saved_read = self.read_position;
        let saved_ch = self.ch;
        let saved_line = self.line;
        let saved_column = self.column;

        self.skip_whitespace();

        // পরের word পড়া
        if self.ch.is_ascii_alphabetic() || self.ch == b'_' || self.is_unicode_bengali_letter() {
            let next_word = self.read_identifier();
            let candidate = format!("{} {}", literal, next_word);
            let candidate_type = lookup_ident(&candidate);

            // যদি lookup match না করে, rewind
            if candidate_type != TokenType::Ident {
                literal = candidate;
                token_type = candidate_type;
            } else {
                self.position = saved_pos;
                self.read_position = saved_read;
                self.ch = saved_ch;
                self.line = saved_line;
                self.column = saved_column;
                break;
            }
        } else {
            break;
        }
    }

    Token::new(token_type, &literal, self.token_start_line, self.token_start_column)
}


            b'0'..=b'9' => {
                let (literal, token_type) = self.read_number();
                return Token::new(token_type, &literal, self.token_start_line, self.token_start_column);
            }

            0 => Token::new(TokenType::Eof, "", self.token_start_line, self.token_start_column),
            _ => Token::new(TokenType::Illegal, &(self.ch as char).to_string(), self.token_start_line, self.token_start_column),
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
                    self.read_char();
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
        let start = self.position + 1;
        let end = (start + n).min(self.input.len());

        if start >= self.input.len() {
            return String::new();
        }

        // Avoid allocation per char by using iterator
        self.input[start..end].to_string()
    }

    fn read_identifier(&mut self) -> String {
        let start_pos = self.position;

        while self.ch.is_ascii_alphabetic() || self.ch == b'_' || self.is_unicode_bengali_letter() {
            self.read_char();
        }

        self.input[start_pos..self.position].to_string()
    }

    fn read_number(&mut self) -> (String, TokenType) {
        let start_pos = self.position;
        let mut has_dot = false;
        let mut has_exp = false;
        let mut has_i = false;
        let mut token_type = TokenType::Int;

        while {
            let c = self.ch as char;
            if c.is_ascii_digit() {
                true
            } else if c == '.' && !has_dot && !has_i {
                has_dot = true;
                token_type = TokenType::Float;
                true
            } else if (c == 'e' || c == 'E') && !has_exp && !has_i {
                has_exp = true;
                token_type = TokenType::Double;
                true
            } else if (c == '+' || c == '-') && has_exp {
                true
            } else if c == 'i' && !has_i {
                has_i = true;
                token_type = TokenType::Complex;
                true
            } else if c == 'm' || c == 'M' {
                token_type = TokenType::Decimal;
                true
            } else {
                false
            }
        } {
            self.read_char();
        }

        (self.input[start_pos..self.position].to_string(), token_type)
    }

    fn read_string(&mut self) -> Result<String, String> {
        self.read_char(); // consume opening quote

        let mut result = String::new();

        while self.ch != b'"' && self.ch != 0 {
            if self.ch == b'\\' {
                self.read_char();
                let escaped_char = match self.ch {
                    b'n' => '\n',
                    b't' => '\t',
                    b'r' => '\r',
                    b'"' => '"',
                    b'\\' => '\\',
                    other => other as char,
                };
                result.push(escaped_char);
            } else {
                result.push(self.ch as char);
            }
            self.read_char();
        }

        if self.ch == b'"' {
            self.read_char(); // consume closing quote
            Ok(result)
        } else {
            Err("Unterminated string literal".to_string())
        }
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }

    fn is_unicode_bengali_letter(&self) -> bool {
        if self.position >= self.input.len() {
            return false;
        }

        let s = &self.input[self.position..];
        if let Some(ch) = s.chars().next() {
            // Bengali Unicode block range: U+0980 to U+09FF
            (ch >= '\u{0980}' && ch <= '\u{09FF}')
        } else {
            false
        }
    }
}
