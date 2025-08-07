// compiler/src/lexer.rs

use crate::token::{lookup_ident, Token, TokenType};

pub struct Lexer {
    input: String,
    position: usize,      // Current index in input string (points to current char)
    read_position: usize, // Next index to read from input (after current char)
    ch: u8,               // Current byte (character) under examination
}

impl Lexer {
    // Create a new Lexer instance with input source code
    pub fn new(input: String) -> Self {
        let mut l = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: 0,
        };
        l.read_char(); // Initialize by reading first character
        l
    }

    // Advance reading position by one character and update current char
    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0; // End of file/input indicated by 0
        } else {
            self.ch = self.input.as_bytes()[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    // Peek at the next character without advancing read position
    fn peek_char(&self) -> u8 {
        if self.read_position >= self.input.len() {
            0
        } else {
            self.input.as_bytes()[self.read_position]
        }
    }

    // Return the next token from the input source
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        // Handle different styles of comments before tokenizing
        if self.ch == b'/' {
            if self.peek_char() == b'/' {
                // Single-line comment starting with //
                self.read_char();
                self.read_char();
                self.skip_single_line_comment();
                return self.next_token();
            } else if self.peek_char() == b'*' {
                // Multi-line comment starting with /*
                self.read_char();
                self.read_char();
                if let Err(err) = self.skip_multi_line_comment("/*", "*/") {
                    return Token::new(TokenType::Illegal, &err);
                }
                return self.next_token();
            }
        } else if self.ch == b'#' {
            // Single-line comment starting with #
            self.read_char();
            self.skip_single_line_comment();
            return self.next_token();
        } else if self.ch == b'-' && self.peek_char() == b'-' {
            // Single-line comment starting with --
            self.read_char();
            self.read_char();
            self.skip_single_line_comment();
            return self.next_token();
        } else if self.ch == b'=' {
            // Ruby-style multiline comment =begin ... =end
            let lookahead = self.peek_n_chars(5);
            if lookahead == "begin" {
                // Consume "=begin"
                for _ in 0..6 { self.read_char(); }
                if let Err(err) = self.skip_multi_line_comment("=begin", "=end") {
                    return Token::new(TokenType::Illegal, &err);
                }
                return self.next_token();
            }
        } else if self.ch == b'{' && self.peek_char() == b'-' {
            // Haskell-style multiline comment {- ... -}
            self.read_char();
            self.read_char();
            if let Err(err) = self.skip_multi_line_comment("{-", "-}") {
                return Token::new(TokenType::Illegal, &err);
            }
            return self.next_token();
        } else if self.ch == b'(' && self.peek_char() == b'*' {
            // OCaml/Pascal-style multiline comment (* ... *)
            self.read_char();
            self.read_char();
            if let Err(err) = self.skip_multi_line_comment("(*", "*)") {
                return Token::new(TokenType::Illegal, &err);
            }
            return self.next_token();
        } else if self.ch == b'"' {
            // Python triple-quote docstring """ ... """
            let lookahead = self.peek_n_chars(2);
            if lookahead == "\"\"" {
                // Consume opening """
                self.read_char();
                self.read_char();
                self.read_char();
                if let Err(err) = self.skip_multi_line_comment("\"\"\"", "\"\"\"") {
                    return Token::new(TokenType::Illegal, &err);
                }
                return self.next_token();
            }
        } else if self.ch == b'\'' {
            // Python triple single-quote docstring ''' ... '''
            let lookahead = self.peek_n_chars(2);
            if lookahead == "''" {
                // Consume opening '''
                self.read_char();
                self.read_char();
                self.read_char();
                if let Err(err) = self.skip_multi_line_comment("'''", "'''") {
                    return Token::new(TokenType::Illegal, &err);
                }
                return self.next_token();
            }
        }

        // Match single-character or multi-character tokens
        let tok = match self.ch {
            b'=' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    Token::new(TokenType::Eq, "==") // Equality operator
                } else {
                    Token::new(TokenType::Assign, "=") // Assignment operator
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
                    Token::new(TokenType::NotEq, "!=") // Not equal operator
                } else {
                    Token::new(TokenType::Bang, "!") // Logical NOT
                }
            }
            b'/' => Token::new(TokenType::Slash, "/"),
            b'*' => Token::new(TokenType::Asterisk, "*"),
            b'<' => Token::new(TokenType::Lt, "<"),
            b'>' => Token::new(TokenType::Gt, ">"),
            b'{' => Token::new(TokenType::LBrace, "{"),
            b'}' => Token::new(TokenType::RBrace, "}"),
            b'"' => {
                // Read string literal enclosed in double quotes
                let literal = self.read_string();
                return Token::new(TokenType::String, &literal);
            }
            b'.' => Token::new(TokenType::Fullstop, "."), // Fullstop token added

            // Identifiers or multiword keywords (allow Bengali Unicode letters too)
            _ if self.ch.is_ascii_alphabetic() || self.ch == b'_' || self.is_unicode_bengali_letter() => {
                // Read first identifier
                let first_literal = self.read_identifier();

                self.skip_whitespace();

                // Save state to rewind if next word is not a valid multi-word keyword
                let saved_position = self.position;
                let saved_read_position = self.read_position;
                let saved_ch = self.ch;

                let mut multi_word_literal = first_literal.clone();

                // Check for possible multi-word keyword
                if self.ch.is_ascii_alphabetic() || self.ch == b'_' || self.is_unicode_bengali_letter() {
                    let second_literal = self.read_identifier();
                    multi_word_literal = format!("{} {}", multi_word_literal, second_literal);

                    // Lookup token type for multi-word literal
                    let token_type = lookup_ident(&multi_word_literal);
                    if token_type != TokenType::Ident {
                        return Token::new(token_type, &multi_word_literal);
                    } else {
                        // Rewind lexer state if not matched
                        self.position = saved_position;
                        self.read_position = saved_read_position;
                        self.ch = saved_ch;
                    }
                }

                // Return token for first identifier
                let token_type = lookup_ident(&first_literal);
                return Token::new(token_type, &first_literal);
            }

            b'0'..=b'9' => {
                // Read integer literals (only digits)
                let literal = self.read_number();
                return Token::new(TokenType::Int, &literal);
            }
            0 => Token::new(TokenType::Eof, ""), // End of input
            _ => Token::new(TokenType::Illegal, &(self.ch as char).to_string()), // Illegal/unknown token
        };

        self.read_char();
        tok
    }

    // Skip characters until end of single line (newline or EOF)
    fn skip_single_line_comment(&mut self) {
        while self.ch != b'\n' && self.ch != 0 {
            self.read_char();
        }
    }

    // Skip a multi-line comment bounded by start and end delimiters
    fn skip_multi_line_comment(&mut self, start: &str, end: &str) -> Result<(), String> {
        let mut end_matched = 0;
        let end_bytes = end.as_bytes();
        let end_len = end_bytes.len();

        loop {
            if self.ch == 0 {
                // EOF reached before end delimiter - unterminated comment
                return Err(format!("Unterminated multi-line comment starting with {}", start));
            }
            if self.ch == end_bytes[end_matched] {
                end_matched += 1;
                if end_matched == end_len {
                    self.read_char(); // Consume last char of end delimiter
                    break;
                }
            } else {
                end_matched = 0; // Reset if sequence interrupted
            }
            self.read_char();
        }
        Ok(())
    }

    // Peek ahead n characters and return as String without advancing lexer state
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

    // Read an identifier: sequence of letters, underscores or Bengali Unicode letters
    fn read_identifier(&mut self) -> String {
        let start_pos = self.position;

        while self.ch.is_ascii_alphabetic() || self.ch == b'_' || self.is_unicode_bengali_letter() {
            self.read_char();
        }

        self.input[start_pos..self.position].to_string()
    }

    // Read a number literal: sequence of ASCII digits
    fn read_number(&mut self) -> String {
        let start_pos = self.position;

        while self.ch.is_ascii_digit() {
            self.read_char();
        }

        self.input[start_pos..self.position].to_string()
    }

    // Read a string literal enclosed in double quotes
    fn read_string(&mut self) -> String {
        let start_pos = self.position + 1; // Skip opening quote

        loop {
            self.read_char();
            if self.ch == b'"' || self.ch == 0 {
                break;
            }
        }

        let result = self.input[start_pos..self.position].to_string();

        self.read_char(); // Skip closing quote

        result
    }

    // Skip whitespace characters (space, tab, newline, etc.)
    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }

    /// Check if current char starts a Bengali Unicode letter (3-byte UTF-8)
    /// Bengali Unicode range: U+0980 to U+09FF encoded in UTF-8 as 3 bytes
    fn is_unicode_bengali_letter(&self) -> bool {
        // Ensure enough bytes to check 3-byte sequence
        if self.position + 3 > self.input.len() {
            return false;
        }
        let bytes = &self.input.as_bytes()[self.position..self.position + 3];
        // UTF-8 Bengali letters start with 0xE0, second byte between 0xA6-0xAF,
        // and third byte between 0x80-0xBF
        bytes[0] == 0xE0 && (0xA6..=0xAF).contains(&bytes[1]) && (0x80..=0xBF).contains(&bytes[2])
    }
}
