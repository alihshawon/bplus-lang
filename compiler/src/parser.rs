// compiler/src/parser.rs

use crate::ast::{Expression, Program, Statement};
use crate::lexer::Lexer;
use crate::token::{Token, TokenType};
use std::collections::HashMap;
use std::io::{self, Write};

#[derive(PartialEq, PartialOrd)]
enum Precedence {
    LOWEST,
    EQUALS,      // ==
    LESSGREATER, // > or <
    SUM,         // +
    PRODUCT,     // *
    PREFIX,      // -X or !X
    CALL,        // myFunction(X)
}

type PrefixParseFn = fn(&mut Parser) -> Option<Expression>;
type InfixParseFn = fn(&mut Parser, Expression) -> Option<Expression>;

pub struct Parser {
    lexer: Lexer,
    cur_token: Token,
    peek_token: Token,
    pub errors: Vec<String>,
    prefix_parse_fns: HashMap<TokenType, PrefixParseFn>,
    infix_parse_fns: HashMap<TokenType, InfixParseFn>,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let mut p = Parser {
            lexer,
            cur_token: Token::new(TokenType::Illegal, ""),
            peek_token: Token::new(TokenType::Illegal, ""),
            errors: Vec::new(),
            prefix_parse_fns: HashMap::new(),
            infix_parse_fns: HashMap::new(),
        };

        // Register prefix parsing functions
        p.register_prefix(TokenType::Ident, Self::parse_identifier);
        p.register_prefix(TokenType::Int, Self::parse_integer_literal);
        p.register_prefix(TokenType::String, Self::parse_string_literal);
        p.register_prefix(TokenType::Bang, Self::parse_prefix_expression);
        p.register_prefix(TokenType::Minus, Self::parse_prefix_expression);
        p.register_prefix(TokenType::Ha, Self::parse_boolean);
        p.register_prefix(TokenType::Na, Self::parse_boolean);
        p.register_prefix(TokenType::Jodi, Self::parse_if_expression);
        p.register_prefix(TokenType::LParen, Self::parse_grouped_expression);
        p.register_prefix(TokenType::Function, Self::parse_function_literal);

        // Register infix parsing functions
        p.register_infix(TokenType::Plus, Self::parse_infix_expression);
        p.register_infix(TokenType::Minus, Self::parse_infix_expression);
        p.register_infix(TokenType::Slash, Self::parse_infix_expression);
        p.register_infix(TokenType::Asterisk, Self::parse_infix_expression);
        p.register_infix(TokenType::Eq, Self::parse_infix_expression);
        p.register_infix(TokenType::NotEq, Self::parse_infix_expression);
        p.register_infix(TokenType::Lt, Self::parse_infix_expression);
        p.register_infix(TokenType::Gt, Self::parse_infix_expression);
        p.register_infix(TokenType::LParen, Self::parse_call_expression);

        p.next_token();
        p.next_token();
        p
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program: Program = Vec::new();
        while self.cur_token.token_type != TokenType::Eof {
            if let Some(stmt) = self.parse_statement() {
                program.push(stmt);
            }
            self.next_token();
        }
        program
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.cur_token.token_type {
            TokenType::Dhoro => self.parse_let_statement(),
            TokenType::Ferot => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_let_statement(&mut self) -> Option<Statement> {
        if !self.expect_peek(TokenType::Ident) {
            return None;
        }

        let name = Expression::Identifier(self.cur_token.literal.clone());

        if !self.expect_peek(TokenType::Assign) {
            return None;
        }

        self.next_token();

        let value = self.parse_expression(Precedence::LOWEST)?;

        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }

        Some(Statement::Let { name, value })
    }

    fn parse_return_statement(&mut self) -> Option<Statement> {
        self.next_token();
        let return_value = self.parse_expression(Precedence::LOWEST)?;
        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }
        Some(Statement::Return { return_value })
    }

    fn parse_expression_statement(&mut self) -> Option<Statement> {
        let expression = self.parse_expression(Precedence::LOWEST)?;

        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }

        Some(Statement::ExpressionStatement { expression })
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Option<Expression> {
        let prefix_fn = self.prefix_parse_fns.get(&self.cur_token.token_type);

        if prefix_fn.is_none() {
            self.no_prefix_parse_fn_error(self.cur_token.token_type.clone());
            return None;
        }

        let mut left_exp = prefix_fn.unwrap()(self)?;

        while !self.peek_token_is(TokenType::Semicolon) && precedence < self.peek_precedence() {
            let infix_fn = self.infix_parse_fns.get(&self.peek_token.token_type).cloned();
            if infix_fn.is_none() {
                return Some(left_exp);
            }
            self.next_token();
            left_exp = infix_fn.unwrap()(self, left_exp)?;
        }

        Some(left_exp)
    }

    // Prefix parsing functions

    fn parse_identifier(&mut self) -> Option<Expression> {
        Some(Expression::Identifier(self.cur_token.literal.clone()))
    }

    fn parse_integer_literal(&mut self) -> Option<Expression> {
        match self.cur_token.literal.parse::<i64>() {
            Ok(value) => Some(Expression::IntegerLiteral(value)),
            Err(_) => {
                self.errors.push(format!("could not parse {} as integer", self.cur_token.literal));
                None
            }
        }
    }

    fn parse_string_literal(&mut self) -> Option<Expression> {
        Some(Expression::StringLiteral(self.cur_token.literal.clone()))
    }

    fn parse_boolean(&mut self) -> Option<Expression> {
        Some(Expression::Boolean(self.cur_token.token_type == TokenType::Ha))
    }

    fn parse_prefix_expression(&mut self) -> Option<Expression> {
        let operator = self.cur_token.literal.clone();
        self.next_token();
        let right = self.parse_expression(Precedence::PREFIX)?;
        Some(Expression::Prefix { operator, right: Box::new(right) })
    }

    fn parse_grouped_expression(&mut self) -> Option<Expression> {
        self.next_token();
        let exp = self.parse_expression(Precedence::LOWEST);
        if !self.expect_peek(TokenType::RParen) {
            return None;
        }
        exp
    }

    fn parse_if_expression(&mut self) -> Option<Expression> {
        self.next_token(); // consume 'jodi'

        let condition = if self.cur_token_is(TokenType::LParen) {
            self.next_token();
            let cond = self.parse_expression(Precedence::LOWEST)?;
            if !self.expect_peek(TokenType::RParen) {
                return None;
            }
            cond
        } else {
            self.parse_expression(Precedence::LOWEST)?
        };

        // accept optional 'hoy', 'tahole', and comma tokens after condition
        self.accept_optional_keywords(&[TokenType::Hoy, TokenType::Tahole, TokenType::Comma]);

        let consequence = if self.peek_token_is(TokenType::LBrace) {
            self.next_token();
            self.parse_block_statement()?
        } else {
            self.next_token();
            let stmt = self.parse_statement().unwrap_or_else(|| {
                self.errors.push("Expected statement after jodi consequence".to_string());
                Statement::ExpressionStatement {
                    expression: Expression::Boolean(false),
                }
            });
            vec![stmt]
        };

        let else_keywords = [
            TokenType::Nahoy,
            TokenType::Noyto,
            TokenType::Noile,
            TokenType::Othoba,
        ];

        let mut alternative = None;
        if else_keywords.iter().any(|&kw| self.peek_token_is(kw)) {
            self.next_token();

            if self.peek_token_is(TokenType::Comma) {
                self.next_token();
            }

            if self.peek_token_is(TokenType::Jodi) {
                self.next_token(); // consume 'jodi'
                // else if এর জন্য recursive call
                alternative = Some(vec![Statement::ExpressionStatement {
                    expression: self.parse_if_expression()?,
                }]);
            } else if self.peek_token_is(TokenType::LBrace) {
                self.next_token();
                alternative = Some(self.parse_block_statement()?);
            } else {
                self.next_token();
                let stmt = self.parse_statement().unwrap_or_else(|| {
                    self.errors.push("Expected statement after else part".to_string());
                    Statement::ExpressionStatement {
                        expression: Expression::Boolean(false),
                    }
                });
                alternative = Some(vec![stmt]);
            };
        }

        Some(Expression::If {
            condition: Box::new(condition),
            consequence,
            alternative,
        })
    }

    fn accept_optional_keywords(&mut self, keywords: &[TokenType]) {
        while keywords.iter().any(|&kw| self.peek_token.token_type == kw) {
            self.next_token();
        }
    }

    fn parse_block_statement(&mut self) -> Option<Vec<Statement>> {
        let mut statements = Vec::new();
        self.next_token(); // consume '{'

        while !self.cur_token_is(TokenType::RBrace) && !self.cur_token_is(TokenType::Eof) {
            if let Some(stmt) = self.parse_statement() {
                statements.push(stmt);
            }
            self.next_token();
        }
        Some(statements)
    }

    fn parse_function_literal(&mut self) -> Option<Expression> {
        if !self.expect_peek(TokenType::LParen) {
            return None;
        }

        let parameters = self.parse_function_parameters()?;

        if !self.expect_peek(TokenType::RParen) { // corrected from RBrace to RParen
            return None;
        }

        if !self.expect_peek(TokenType::LBrace) {
            return None;
        }

        let body = self.parse_block_statement()?;

        Some(Expression::FunctionLiteral { parameters, body })
    }

    fn parse_function_parameters(&mut self) -> Option<Vec<Expression>> {
        let mut identifiers = Vec::new();

        if self.peek_token_is(TokenType::RParen) {
            self.next_token();
            return Some(identifiers);
        }

        self.next_token();

        identifiers.push(Expression::Identifier(self.cur_token.literal.clone()));

        while self.peek_token_is(TokenType::Comma) {
            self.next_token();
            self.next_token();
            identifiers.push(Expression::Identifier(self.cur_token.literal.clone()));
        }

        if !self.expect_peek(TokenType::RParen) {
            return None;
        }

        Some(identifiers)
    }

    // Infix parsing functions

    fn parse_infix_expression(&mut self, left: Expression) -> Option<Expression> {
        let operator = self.cur_token.literal.clone();
        let precedence = self.cur_precedence();
        self.next_token();
        let right = self.parse_expression(precedence)?;
        Some(Expression::Infix { left: Box::new(left), operator, right: Box::new(right) })
    }

    fn parse_call_expression(&mut self, function: Expression) -> Option<Expression> {
        let arguments = self.parse_call_arguments()?;
        Some(Expression::Call { function: Box::new(function), arguments })
    }

    fn parse_call_arguments(&mut self) -> Option<Vec<Expression>> {
        let mut args = Vec::new();

        if self.peek_token_is(TokenType::RParen) {
            self.next_token();
            return Some(args);
        }

        self.next_token();

        if let Some(exp) = self.parse_expression(Precedence::LOWEST) {
            args.push(exp);
        }

        while self.peek_token_is(TokenType::Comma) {
            self.next_token();
            self.next_token();
            if let Some(exp) = self.parse_expression(Precedence::LOWEST) {
                args.push(exp);
            }
        }

        if !self.expect_peek(TokenType::RParen) {
            return None;
        }

        Some(args)
    }

    // Helper methods

    fn cur_token_is(&self, t: TokenType) -> bool {
        self.cur_token.token_type == t
    }

    fn peek_token_is(&self, t: TokenType) -> bool {
        self.peek_token.token_type == t
    }

    fn expect_peek(&mut self, t: TokenType) -> bool {
        if self.peek_token_is(t.clone()) {
            self.next_token();
            true
        } else {
            self.peek_error(t);
            false
        }
    }

    fn peek_error(&mut self, t: TokenType) {
        self.errors.push(format!(
            "expected next token to be {:?}, got {:?} instead",
            t, self.peek_token.token_type
        ));
    }

    fn no_prefix_parse_fn_error(&mut self, t: TokenType) {
        self.errors.push(format!("no prefix parse function for {:?} found", t));
    }

    fn get_precedence(&self, t: &TokenType) -> Precedence {
        match t {
            TokenType::Eq | TokenType::NotEq => Precedence::EQUALS,
            TokenType::Lt | TokenType::Gt => Precedence::LESSGREATER,
            TokenType::Plus | TokenType::Minus => Precedence::SUM,
            TokenType::Slash | TokenType::Asterisk => Precedence::PRODUCT,
            TokenType::LParen => Precedence::CALL,
            _ => Precedence::LOWEST,
        }
    }

    fn peek_precedence(&self) -> Precedence {
        self.get_precedence(&self.peek_token.token_type)
    }

    fn cur_precedence(&self) -> Precedence {
        self.get_precedence(&self.cur_token.token_type)
    }

    fn register_prefix(&mut self, token_type: TokenType, func: PrefixParseFn) {
        self.prefix_parse_fns.insert(token_type, func);
    }

    fn register_infix(&mut self, token_type: TokenType, func: InfixParseFn) {
        self.infix_parse_fns.insert(token_type, func);
    }

    // Interactive mode for REPL

    pub fn run_interactive_mode(&mut self) {
        let mut input = String::new();
        loop {
            print!("> ");
            io::stdout().flush().unwrap();
            input.clear();
            if io::stdin().read_line(&mut input).is_err() {
                eprintln!("Error reading input");
                continue;
            }
            let trimmed_input = input.trim();
            if trimmed_input.is_empty() {
                continue;
            }
            if !Self::brackets_balanced(trimmed_input) {
                eprintln!("Unbalanced brackets in input");
                continue;
            }
            Self::run_source(trimmed_input);
        }
    }

    // Placeholder for bracket balance checking; implement as needed
    fn brackets_balanced(_input: &str) -> bool {
        true
    }

    // Placeholder for running source code input; implement as needed
    fn run_source(_source: &str) {
        // Implement your source code execution here
    }
}
