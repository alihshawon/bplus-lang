// compiler/src/ast.rs

// === IMPORTS ===
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
        name: Expression,  // Variable name as Expression::Identifier
        value: Expression, // Right-hand side expression
        mutable: bool, // Mutable flag
    },

    Assign { 
        name: Expression, 
        value: Expression 
    },
    
    Expression(Expression),

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
            Statement::Let { name, value, mutable } =>
                if *mutable {
                        write!(f, "dhoro {} = {};", name, value)
                    } else {
                        write!(f, "let {} = {};", name, value)
                    },
            Statement::Assign { name, value } =>
                write!(f, "{} = {};", name, value),

            Statement::Return { return_value } =>
                write!(f, "return {};", return_value),

            Statement::ExpressionStatement { expression } =>
                write!(f, "{}", expression),

            Statement::CommentSingleLine { content } =>
                write!(f, "//{}", content),

            Statement::CommentMultiLine { content } =>
                write!(f, "/*{}*/", content),

            Statement::While { condition, body } => {
                let mut s = format!("jotokhon {} {{ ", condition);
                for stmt in body {
                    s.push_str(&format!("{}", stmt));
                }
                s.push_str(" }");
                write!(f, "{}", s)
            }

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

            Statement::Break =>
                write!(f, "thamo;"),

            Statement::Continue =>
                write!(f, "choluk;"),

            Statement::Expression(expr) =>
                write!(f, "{}", expr),
        }
    }
}

// === EXPRESSION ENUM DEFINITION ===
// The 'Expression' enum defines all possible expressions in B+.
#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Identifier(String),            // e.g., variable name

    IntegerLiteral(i64),           // e.g., 123

    StringLiteral(String),         // e.g., "hello"

    Boolean(bool),                 // Ha / Na

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
        consequence: Vec<Statement>,
        alternative: Option<Box<Expression>>,
    },

    FunctionLiteral {
        parameters: Vec<Expression>,
        body: Vec<Statement>,
    },

    Call {
        function: Box<Expression>,
        arguments: Vec<Expression>,
    },

    TemplateLiteral {
        parts: Vec<Expression>,
    },
}

// === EXPRESSION DISPLAY IMPLEMENTATION ===
// Converts expression enums into proper source code text
impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expression::Identifier(s) =>
                write!(f, "{}", s),

            Expression::IntegerLiteral(i) =>
                write!(f, "{}", i),

            Expression::StringLiteral(s) =>
                write!(f, "\"{}\"", s),

            Expression::Boolean(b) => {
                let s = if *b { "Ha" } else { "Na" };
                write!(f, "{}", s)
            }

            Expression::Prefix { operator, right } =>
                write!(f, "({}{})", operator, right),

            Expression::Infix { left, operator, right } =>
                write!(f, "({} {} {})", left, operator, right),

            Expression::If { condition, consequence, alternative } => {
                let mut s = format!("jodi {} {{ ", condition);
                for stmt in consequence {
                    s.push_str(&format!("{}", stmt));
                }
                s.push_str(" }");

                if let Some(alt_expr) = alternative {
                    match alt_expr.as_ref() {
                        Expression::If { .. } => {
                            s.push_str(" nahoy ");
                            s.push_str(&format!("{}", alt_expr));
                        }
                        _ => {
                            s.push_str(" nahoy { ");
                            s.push_str(&format!("{}", alt_expr));
                            s.push_str(" }");
                        }
                    }
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

            Expression::TemplateLiteral { parts } => {
                let rendered: Vec<String> = parts.iter().map(|p| format!("{}", p)).collect();
                // backticks style
                write!(f, "`{}`", rendered.join(""))
            }
           
        }
    }
}
