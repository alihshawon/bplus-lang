// compiler/src/parser.rs

use crate::ast::{Expression, Program, Statement};
use crate::lexer::Lexer;
use crate::token::{Token, TokenType};
use std::collections::HashMap;

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

        // Read two tokens, so cur_token and peek_token are both set
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

    // --- Prefix Parsing Functions ---

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
        if !self.expect_peek(TokenType::LParen) {
            return None;
        }
        self.next_token();
        let condition = self.parse_expression(Precedence::LOWEST)?;
        if !self.expect_peek(TokenType::RParen) {
            return None;
        }
        if !self.expect_peek(TokenType::LBrace) {
            return None;
        }
        let consequence = self.parse_block_statement()?;

        let mut alternative = None;
        if self.peek_token_is(TokenType::Nahoy) {
            self.next_token();
            if !self.expect_peek(TokenType::LBrace) {
                return None;
            }
            alternative = Some(self.parse_block_statement()?);
        }

        Some(Expression::If {
            condition: Box::new(condition),
            consequence,
            alternative,
        })
    }

    fn parse_block_statement(&mut self) -> Option<Vec<Statement>> {
        let mut statements = Vec::new();
        self.next_token(); // consume {

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

    // --- Infix Parsing Functions ---

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
    
    // CORRECTED FUNCIION
    fn parse_call_arguments(&mut self) -> Option<Vec<Expression>> {
        let mut args = Vec::new();

        // Handles the case of zero arguments, like dekhao()
        if self.peek_token_is(TokenType::RParen) {
            self.next_token(); // Consume the ')'
            return Some(args);
        }

        // Consume the '(' and get ready to parse the first argument
        self.next_token();

        // Parse the first argument
        if let Some(exp) = self.parse_expression(Precedence::LOWEST) {
            args.push(exp);
        }

        // As long as we see a comma, there are more arguments to parse
        while self.peek_token_is(TokenType::Comma) {
            self.next_token(); // Consume the comma
            self.next_token(); // Move to the next argument's token
            if let Some(exp) = self.parse_expression(Precedence::LOWEST) {
                args.push(exp);
            }
        }

        // After all arguments, we must find a closing ')'
        if !self.expect_peek(TokenType::RParen) {
            return None;
        }

        Some(args)
    }


    // --- Helper Methods ---

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
}