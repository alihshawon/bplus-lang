// compiler/src/parser.rs

// Import necessary modules and types from lexer, AST, and token definitions
use crate::ast::{Expression, Program, Statement};
use crate::lexer::Lexer;
use crate::token::{Token, TokenType};
use std::collections::HashMap;
use std::io::{self, Write};

// Precedence levels for parsing expressions with correct operator binding
#[derive(PartialEq, PartialOrd, Debug)]
enum Precedence {
    LOWEST,
    EQUALS,      // == operator
    LESSGREATER, // > or < operators
    SUM,         // + operator
    PRODUCT,     // * operator
    PREFIX,      // -X or !X prefix operators
    CALL,        // Function call like myFunction(X)
}

// Type aliases for prefix and infix parsing function signatures
type PrefixParseFn = fn(&mut Parser) -> Option<Expression>;
type InfixParseFn = fn(&mut Parser, Expression) -> Option<Expression>;

// Parser struct holds lexer, current and peek tokens, errors and registered parse functions
pub struct Parser {
    lexer: Lexer,
    cur_token: Token,
    peek_token: Token,
    pub errors: Vec<String>,
    prefix_parse_fns: HashMap<TokenType, PrefixParseFn>,
    infix_parse_fns: HashMap<TokenType, InfixParseFn>,
}

impl Parser {
    // Create a new Parser instance and register prefix and infix parse functions
    pub fn new(lexer: Lexer) -> Self {
        let mut p = Parser {
            lexer,
cur_token: Token::new(TokenType::Illegal, "", 0, 0),
peek_token: Token::new(TokenType::Illegal, "", 0, 0),

            errors: Vec::new(),
            prefix_parse_fns: HashMap::new(),
            infix_parse_fns: HashMap::new(),
        };

        // Register prefix parsing functions for different token types
        p.register_prefix(TokenType::Ident, Self::parse_identifier);
        p.register_prefix(TokenType::Int, Self::parse_integer_literal);
        p.register_prefix(TokenType::String, Self::parse_string_literal);
        p.register_prefix(TokenType::Bang, Self::parse_prefix_expression);
        p.register_prefix(TokenType::Minus, Self::parse_prefix_expression);
        p.register_prefix(TokenType::Ha, Self::parse_boolean);
        p.register_prefix(TokenType::Na, Self::parse_boolean);
        p.register_prefix(TokenType::Jodi, Self::parse_if_expression);
        p.register_prefix(TokenType::Dekhao, Self::parse_print_expression);
        p.register_prefix(TokenType::LParen, Self::parse_grouped_expression);
        p.register_prefix(TokenType::Function, Self::parse_function_literal);
        p.register_prefix(TokenType::InputNao, Self::parse_input_expression);

        // Register infix parsing functions for operators and calls
        p.register_infix(TokenType::Plus, Self::parse_infix_expression);
        p.register_infix(TokenType::Minus, Self::parse_infix_expression);
        p.register_infix(TokenType::Slash, Self::parse_infix_expression);
        p.register_infix(TokenType::Asterisk, Self::parse_infix_expression);
        p.register_infix(TokenType::Eq, Self::parse_infix_expression);
        p.register_infix(TokenType::NotEq, Self::parse_infix_expression);
        p.register_infix(TokenType::Lt, Self::parse_infix_expression);
        p.register_infix(TokenType::Gt, Self::parse_infix_expression);
        p.register_infix(TokenType::Ebong, Self::parse_infix_expression); // Logical AND
        p.register_infix(TokenType::Othoba, Self::parse_infix_expression);    // Logical OR
        p.register_infix(TokenType::LParen, Self::parse_call_expression);

        // Advance tokens twice to initialize cur_token and peek_token
        p.next_token();
        p.next_token();
        p
    }

    // Parse input() function call expression
    fn parse_input_expression(&mut self) -> Option<Expression> {
        let function_name = self.cur_token.literal.clone();

        if !self.expect_peek(TokenType::LParen) {
            self.errors.push(format!("expected '(' after '{}'", function_name));
            return None;
        }

        let args = self.parse_call_arguments()?;

        Some(Expression::Call {
            function: Box::new(Expression::Identifier(function_name)),
            arguments: args,
        })
    }

    // Advance current and peek tokens from lexer
    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    // Parse the entire program (list of statements)
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

    // Parse a statement depending on current token type
    fn parse_statement(&mut self) -> Option<Statement> {
        match self.cur_token.token_type {
            TokenType::Dhoro => self.parse_let_statement(),
            TokenType::ReturnKoro => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    // Parse a let statement
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

    // Parse a return statement
    fn parse_return_statement(&mut self) -> Option<Statement> {
        self.next_token();
        let return_value = self.parse_expression(Precedence::LOWEST)?;
        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }
        Some(Statement::Return { return_value })
    }

    /// Parse expression statement wrapped as Statement
    fn parse_expression_statement(&mut self) -> Option<Statement> {
        let expr = self.parse_expression(Precedence::LOWEST)?;
        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }
        Some(Statement::ExpressionStatement { expression: expr })
    }

    // Parse expression with operator precedence and associativity
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

    // Parse an identifier expression
    fn parse_identifier(&mut self) -> Option<Expression> {
        Some(Expression::Identifier(self.cur_token.literal.clone()))
    }

    // Parse an integer literal expression
    fn parse_integer_literal(&mut self) -> Option<Expression> {
        match self.cur_token.literal.parse::<i64>() {
            Ok(value) => Some(Expression::IntegerLiteral(value)),
            Err(_) => {
                self.errors.push(format!("could not parse {} as integer", self.cur_token.literal));
                None
            }
        }
    }

    // Parse a string literal expression
    fn parse_string_literal(&mut self) -> Option<Expression> {
        Some(Expression::StringLiteral(self.cur_token.literal.clone()))
    }

    // Parse a boolean literal expression (Ha or Na)
    fn parse_boolean(&mut self) -> Option<Expression> {
        Some(Expression::Boolean(self.cur_token.token_type == TokenType::Ha))
    }

    // Parse a prefix expression like !X or -X
    fn parse_prefix_expression(&mut self) -> Option<Expression> {
        let operator = self.cur_token.literal.clone();
        self.next_token();
        let right = self.parse_expression(Precedence::PREFIX)?;
        Some(Expression::Prefix { operator, right: Box::new(right) })
    }

    // Parse print (dekhao) expression

    fn parse_print_expression(&mut self) -> Option<Expression> {
        self.next_token(); // consume 'dekhao'

        // Check the next token to decide what kind of expression we are parsing
        let expr = match self.cur_token.token_type {
            TokenType::LParen => {
                // dekhao(expr) or dekhao (expr)
                let e = self.parse_grouped_expression()?;
                e
            }
            TokenType::LBrace => {
                // dekhao { ... }  -> template literal
                let template_tokens = self.parse_template_literal()?;
                Expression::TemplateLiteral { parts: template_tokens }
            }
            TokenType::String | TokenType::Int | TokenType::Ident | TokenType::Minus | TokenType::Bang => {
                // bare expression or string literal
                self.parse_expression(Precedence::LOWEST)?
            }
            _ => {
                // fallback: treat it as bare expression
                self.parse_expression(Precedence::LOWEST)?
            }
        };

        Some(Expression::Call {
            function: Box::new(Expression::Identifier("dekhao".to_string())),
            arguments: vec![expr],
        })
    }

    // New helper function to parse template literals like {(name) is (status)}
    fn parse_template_literal(&mut self) -> Option<Vec<Expression>> {
        let mut parts = Vec::new();
        self.next_token(); // consume '{'

        while !self.cur_token_is(TokenType::RBrace) && !self.cur_token_is(TokenType::Eof) {
            match self.cur_token.token_type {
                TokenType::LParen => {
                    // expression inside parentheses
                    self.next_token();
                    let expr = self.parse_expression(Precedence::LOWEST)?;
                    if !self.expect_peek(TokenType::RParen) {
                        return None;
                    }
                    parts.push(expr);
                }
                TokenType::Ident | TokenType::String | TokenType::Int => {
                    // literal text outside parentheses
                    parts.push(Expression::StringLiteral(self.cur_token.literal.clone()));
                }
                _ => {}
            }
            self.next_token();
        }

        Some(parts)
    }





    // Parse grouped expression like (expr)
    fn parse_grouped_expression(&mut self) -> Option<Expression> {
        self.next_token();
        let exp = self.parse_expression(Precedence::LOWEST);
        if !self.expect_peek(TokenType::RParen) {
            return None;
        }
        exp
    }

    // Parse if expression with optional else/else if parts
    fn parse_if_expression(&mut self) -> Option<Expression> {
        self.next_token(); // consume 'jodi'

        // Parse condition expression with logical operators
        let condition = self.parse_logical_expression(Precedence::LOWEST)?;

        // Accept optional tokens after condition like 'hoy', 'tahole', or comma
        self.accept_optional_keywords(&[
            TokenType::Hoy,
            TokenType::Tahole,
            TokenType::Comma,
        ]);

        // Parse consequence block or single statement
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

        // Parse else or else if alternatives
        let else_keywords = [
            TokenType::Nahoy,
        ];

        let mut alternative: Option<Box<Expression>> = None;

        if else_keywords.iter().any(|&kw| self.peek_token_is(kw)) {
            self.next_token(); // consume else keyword

            if self.peek_token_is(TokenType::Comma) {
                self.next_token(); // consume optional comma
            }

            if self.peek_token_is(TokenType::Jodi) {
                self.next_token(); // consume 'jodi' for else if
                if let Some(expr) = self.parse_if_expression() {
                    alternative = Some(Box::new(expr));
                } else {
                    self.errors.push("Failed to parse else if expression".to_string());
                    return None;
                }
            } else if self.peek_token_is(TokenType::LBrace) {
                self.next_token();
                // Parse block and extract first expression statement as else alternative
                let stmts = self.parse_block_statement()?;
                if !stmts.is_empty() {
                    match &stmts[0] {
                        Statement::ExpressionStatement { expression } => {
                            alternative = Some(Box::new(expression.clone()));
                        }
                        _ => {
                            self.errors.push("Expected expression statement inside else block".to_string());
                            return None;
                        }
                    }
                }
            } else {
                self.next_token();
                let stmt = self.parse_statement().unwrap_or_else(|| {
                    self.errors.push("Expected statement after else part".to_string());
                    Statement::ExpressionStatement {
                        expression: Expression::Boolean(false),
                    }
                });
                if let Statement::ExpressionStatement { expression } = stmt {
                    alternative = Some(Box::new(expression));
                } else {
                    self.errors.push("Expected expression statement in else part".to_string());
                    return None;
                }
            }
        }

        Some(Expression::If {
            condition: Box::new(condition),
            consequence,
            alternative,
        })
    }

    /// Accept multiple optional keywords in sequence (used for optional tokens)
    fn accept_optional_keywords(&mut self, keywords: &[TokenType]) {
        while keywords.iter().any(|&kw| self.peek_token.token_type == kw) {
            self.next_token();
        }
    }

    // Parse logical expression chain with operators like ebong, ba
    fn parse_logical_expression(&mut self, precedence: Precedence) -> Option<Expression> {
        let prefix_fn = self.prefix_parse_fns.get(&self.cur_token.token_type)?;

        let mut left_exp = prefix_fn(self)?;

        while !self.peek_token_is(TokenType::Semicolon) && precedence < self.peek_precedence() {
            let peek_type = self.peek_token.token_type;

            if !self.infix_parse_fns.contains_key(&peek_type) {
                break;
            }

            // Allow only logical, comparison, and arithmetic operators in logical chain
            if peek_type != TokenType::Ebong && peek_type != TokenType::Othoba
                && peek_type != TokenType::Eq && peek_type != TokenType::NotEq
                && peek_type != TokenType::Lt && peek_type != TokenType::Gt
                && peek_type != TokenType::LtEq && peek_type != TokenType::GtEq
                && peek_type != TokenType::Plus && peek_type != TokenType::Minus
                && peek_type != TokenType::Asterisk && peek_type != TokenType::Slash
            {
                break;
            }

            let infix_fn = self.infix_parse_fns.get(&peek_type).cloned().unwrap();

            self.next_token();
            left_exp = infix_fn(self, left_exp)?;
        }

        Some(left_exp)
    }

    // Parse a block of statements enclosed in braces { }
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

    // Parse a function literal with parameters and body block
    fn parse_function_literal(&mut self) -> Option<Expression> {
        if !self.expect_peek(TokenType::LParen) {
            return None;
        }

        let parameters = self.parse_function_parameters()?;

        if !self.expect_peek(TokenType::RParen) {
            return None;
        }

        if !self.expect_peek(TokenType::LBrace) {
            return None;
        }

        let body = self.parse_block_statement()?;

        Some(Expression::FunctionLiteral { parameters, body })
    }

    // Parse function parameters separated by commas
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

    // Parse infix expressions like 1 + 2 or a == b
    fn parse_infix_expression(&mut self, left: Expression) -> Option<Expression> {
        let operator = self.cur_token.literal.clone();
        let precedence = self.cur_precedence();
        self.next_token();
        let right = self.parse_expression(precedence)?;
        Some(Expression::Infix {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        })
    }

    // Parse function call expression with arguments
    fn parse_call_expression(&mut self, function: Expression) -> Option<Expression> {
        let arguments = self.parse_call_arguments()?;
        Some(Expression::Call {
            function: Box::new(function),
            arguments,
        })
    }

    // Parse list of call arguments separated by commas
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

    // Helper methods for token checks and errors

    fn cur_token_is(&self, t: TokenType) -> bool {
        self.cur_token.token_type == t
    }

    fn peek_token_is(&self, t: TokenType) -> bool {
        self.peek_token.token_type == t
    }

    // Expect next token to be t; if yes, advance tokens; otherwise record error
    fn expect_peek(&mut self, t: TokenType) -> bool {
        if self.peek_token_is(t.clone()) {
            self.next_token();
            true
        } else {
            self.peek_error(t);
            false
        }
    }

    // Record an error for unexpected peek token
    fn peek_error(&mut self, t: TokenType) {
        self.errors.push(format!(
            "expected next token to be {:?}, got {:?} instead",
            t, self.peek_token.token_type
        ));
    }

    // Record error for missing prefix parse function for token
    fn no_prefix_parse_fn_error(&mut self, t: TokenType) {
        self.errors.push(format!("no prefix parse function for {:?} found", t));
    }

    // Map token type to its parsing precedence level
    fn get_precedence(&self, t: &TokenType) -> Precedence {
        match t {
            TokenType::Eq | TokenType::NotEq => Precedence::EQUALS,
            TokenType::Lt | TokenType::Gt => Precedence::LESSGREATER,
            TokenType::Plus | TokenType::Minus => Precedence::SUM,
            TokenType::Slash | TokenType::Asterisk => Precedence::PRODUCT,
            TokenType::LParen => Precedence::CALL,
            TokenType::Ebong => Precedence::EQUALS, // logical AND
            TokenType::Othoba => Precedence::EQUALS,    // logical OR
            _ => Precedence::LOWEST,
        }
    }

    // Get precedence of peek token
    fn peek_precedence(&self) -> Precedence {
        self.get_precedence(&self.peek_token.token_type)
    }

    // Get precedence of current token
    fn cur_precedence(&self) -> Precedence {
        self.get_precedence(&self.cur_token.token_type)
    }

    // Register a prefix parsing function for a token type
    fn register_prefix(&mut self, token_type: TokenType, func: PrefixParseFn) {
        self.prefix_parse_fns.insert(token_type, func);
    }

    // Register an infix parsing function for a token type
    fn register_infix(&mut self, token_type: TokenType, func: InfixParseFn) {
        self.infix_parse_fns.insert(token_type, func);
    }

    // REPL / Interactive mode loop (optional)
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

    // Check if brackets in input are balanced (stub)
    fn brackets_balanced(_input: &str) -> bool {
        true
    }

    // Run source code (stub for actual execution implementation)
    fn run_source(_source: &str) {
        // TODO: implement code execution here
    }
}
