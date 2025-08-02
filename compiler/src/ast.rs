// compiler/src/ast.rs

use std::fmt;

// The top-level structure of a program is a list of statements.
pub type Program = Vec<Statement>;

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Let {
        name: Expression, // Identifier
        value: Expression,
    },
    Return {
        return_value: Expression,
    },
    ExpressionStatement {
        expression: Expression,
    },
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Statement::Let { name, value } => write!(f, "let {} = {};", name, value),
            Statement::Return { return_value } => write!(f, "return {};", return_value),
            Statement::ExpressionStatement { expression } => write!(f, "{}", expression),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Identifier(String),
    IntegerLiteral(i64),
    StringLiteral(String),
    Boolean(bool),
    Prefix {
        operator: String,
        right: Box<Expression>,
    },
    Infix {
        left: Box<Expression>,
        operator: String,
        right: Box<Expression>,
    },
    If {
        condition: Box<Expression>,
        consequence: Vec<Statement>, // Block of statements
        alternative: Option<Vec<Statement>>,
    },
    FunctionLiteral {
        parameters: Vec<Expression>, // List of identifiers
        body: Vec<Statement>,
    },
    Call {
        function: Box<Expression>, // Identifier or FunctionLiteral
        arguments: Vec<Expression>,
    },
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expression::Identifier(s) => write!(f, "{}", s),
            Expression::IntegerLiteral(i) => write!(f, "{}", i),
            Expression::StringLiteral(s) => write!(f, "\"{}\"", s),
            Expression::Boolean(b) => {
                let s = if *b { "Ha" } else { "Na" };
                    write!(f, "{}", s)
            }

            Expression::Prefix { operator, right } => write!(f, "({}{})", operator, right),
            Expression::Infix { left, operator, right } => write!(f, "({} {} {})", left, operator, right),
            Expression::If { condition, consequence, alternative } => {
                let mut s = format!("jodi {} {{ ", condition);
                for stmt in consequence {
                    s.push_str(&format!("{}", stmt));
                }
                s.push_str(" }");
                if let Some(alt) = alternative {
                    s.push_str(" nahoy { ");
                    for stmt in alt {
                        s.push_str(&format!("{}", stmt));
                    }
                    s.push_str(" }");
                }
                write!(f, "{}", s)
            }
            Expression::FunctionLiteral { parameters, body } => {
                let params: Vec<String> = parameters.iter().map(|p| format!("{}", p)).collect();
                let mut s = format!("fn({}) {{ ", params.join(", "));
                for stmt in body {
                    s.push_str(&format!("{}", stmt));
                }
                s.push_str(" }");
                write!(f, "{}", s)
            }
            Expression::Call { function, arguments } => {
                let args: Vec<String> = arguments.iter().map(|a| format!("{}", a)).collect();
                write!(f, "{}({})", function, args.join(", "))
            }
        }
    }
}