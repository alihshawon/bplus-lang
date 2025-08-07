// compiler/src/ast.rs

// === IMPORTS ===
// Importing formatting trait for implementing Display
use std::fmt;

// === PROGRAM STRUCTURE ===
// A program is a list of statements, represented as a vector.
pub type Program = Vec<Statement>;


// === STATEMENT ENUM DEFINITION ===
// The 'Statement' enum defines all types of statements our language supports.
#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    // Variable declaration: let <name> = <value>;
    Let {
        name: Expression, // Variable name (as an Expression::Identifier)
        value: Expression, // Right-hand side expression
    },

    // Return statement: return <value>;
    Return {
        return_value: Expression,
    },

    // Standalone expression: e.g., function call or literal
    ExpressionStatement {
        expression: Expression,
    },

    // Single-line comment: // this is a comment
    CommentSingleLine {
        content: String,
    },

    // Multi-line comment: /* this is a multi-line comment */
    CommentMultiLine {
        content: String,
    },

    // While loop: jotokhon <condition> { <body> }
    While {
        condition: Expression,
        body: Vec<Statement>,
    },

    // For loop: jonno (<init>; <condition>; <update>) { <body> }
    For {
        init: Option<Box<Statement>>,         // Initialization
        condition: Option<Expression>,        // Loop condition
        update: Option<Expression>,           // Update step
        body: Vec<Statement>,                 // Loop body
    },

    // Break statement: thamo;
    Break,

    // Continue statement: choluk;
    Continue,
}


// === STATEMENT DISPLAY IMPLEMENTATION ===
// This lets statements be printed as B+ code
impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            // let <name> = <value>;
            Statement::Let { name, value } => write!(f, "let {} = {};", name, value),

            // return <value>;
            Statement::Return { return_value } => write!(f, "return {};", return_value),

            // Expression (e.g., x + y;)
            Statement::ExpressionStatement { expression } => write!(f, "{}", expression),

            // Single-line comment: //comment
            Statement::CommentSingleLine { content } => write!(f, "//{}", content),

            // Multi-line comment: /*comment*/
            Statement::CommentMultiLine { content } => write!(f, "/*{}*/", content),

            // While loop: jotokhon condition { ... }
            Statement::While { condition, body } => {
                let mut s = format!("jotokhon {} {{ ", condition);
                for stmt in body {
                    s.push_str(&format!("{}", stmt));
                }
                s.push_str(" }");
                write!(f, "{}", s)
            }

            // For loop: jonno (init; condition; update) { ... }
            Statement::For { init, condition, update, body } => {
                let mut s = String::from("jonno (");

                if let Some(init) = init {
                    s.push_str(&format!("{}", init));
                }
                s.push_str("; ");

                if let Some(cond) = condition {
                    s.push_str(&format!("{}", cond));
                }
                s.push_str("; ");

                if let Some(upd) = update {
                    s.push_str(&format!("{}", upd));
                }
                s.push_str(") { ");

                for stmt in body {
                    s.push_str(&format!("{}", stmt));
                }
                s.push_str(" }");
                write!(f, "{}", s)
            }

            // Break: thamo;
            Statement::Break => write!(f, "thamo;"),

            // Continue: choluk;
            Statement::Continue => write!(f, "choluk;"),
        }
    }
}


// === EXPRESSION ENUM DEFINITION ===
// The 'Expression' enum defines all possible expressions in B+.
#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    // Identifier (e.g., variable name)
    Identifier(String),

    // Integer constant (e.g., 123)
    IntegerLiteral(i64),

    // String literal (e.g., "hello")
    StringLiteral(String),

    // Boolean: Ha (true) or Na (false)
    Boolean(bool),

    // Prefix expression: !x, -y, etc.
    Prefix {
        operator: String,
        right: Box<Expression>,
    },

    // Infix expression: x + y, a * b, etc.
    Infix {
        left: Box<Expression>,
        operator: String,
        right: Box<Expression>,
    },

    // If-else or if-else-if expression
    If {
        condition: Box<Expression>,
        consequence: Vec<Statement>,            // Block to run if condition is true
        alternative: Option<Box<Expression>>,   // Else block OR nested If expression
    },

    // Function definition: fn(x, y) { ... }
    FunctionLiteral {
        parameters: Vec<Expression>, // List of function parameters (identifiers)
        body: Vec<Statement>,        // Function body
    },

    // Function call: add(1, 2)
    Call {
        function: Box<Expression>,      // What to call
        arguments: Vec<Expression>,     // Arguments to pass
    },
}


// === EXPRESSION DISPLAY IMPLEMENTATION ===
// Converts expression enums into proper source code text
impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            // Identifier: e.g., x
            Expression::Identifier(s) => write!(f, "{}", s),

            // Integer: e.g., 123
            Expression::IntegerLiteral(i) => write!(f, "{}", i),

            // String: e.g., "hello"
            Expression::StringLiteral(s) => write!(f, "\"{}\"", s),

            // Boolean: Ha / Na
            Expression::Boolean(b) => {
                let s = if *b { "Ha" } else { "Na" };
                write!(f, "{}", s)
            }

            // Prefix: (!x), (-y)
            Expression::Prefix { operator, right } => write!(f, "({}{})", operator, right),

            // Infix: (x + y), (a * b)
            Expression::Infix { left, operator, right } => write!(f, "({} {} {})", left, operator, right),

            // If expression (with optional else or else-if)
            Expression::If { condition, consequence, alternative } => {
                let mut s = format!("jodi {} {{ ", condition);
                for stmt in consequence {
                    s.push_str(&format!("{}", stmt));
                }
                s.push_str(" }");

                if let Some(alt_expr) = alternative {
                    match alt_expr.as_ref() {
                        Expression::If { .. } => {
                            // else if (nahoy jodi ...)
                            s.push_str(" nahoy ");
                            s.push_str(&format!("{}", alt_expr));
                        }
                        _ => {
                            // else block
                            s.push_str(" nahoy { ");
                            s.push_str(&format!("{}", alt_expr));
                            s.push_str(" }");
                        }
                    }
                }

                write!(f, "{}", s)
            }

            // Function literal: fn(x, y) { ... }
            Expression::FunctionLiteral { parameters, body } => {
                let params: Vec<String> = parameters.iter().map(|p| format!("{}", p)).collect();
                let mut s = format!("fn({}) {{ ", params.join(", "));
                for stmt in body {
                    s.push_str(&format!("{}", stmt));
                }
                s.push_str(" }");
                write!(f, "{}", s)
            }

            // Function call: add(1, 2)
            Expression::Call { function, arguments } => {
                let args: Vec<String> = arguments.iter().map(|a| format!("{}", a)).collect();
                write!(f, "{}({})", function, args.join(", "))
            }
        }
    }
}
