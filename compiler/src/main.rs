// compiler/src/main.rs

// Removed unused module declarations
mod ast;
mod environment;
mod evaluator;
mod lexer;
mod object;
mod parser;
mod token;

use environment::Environment;
use lexer::Lexer;
use parser::Parser;
use std::io::{self, Write};

// The main function now starts an interactive REPL
fn main() {
    println!("Welcome to the B+ REPL!");
    println!("You can now use Bangla keywords.");
    // Updated example with new keywords
    println!("Try: jodi (10 > 5) {{ dekhao(\"10 is greater than 5!\") }}");

    let mut env = Environment::new();

    loop {
        print!(">> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            break;
        }

        if input.trim() == "exit" {
            break;
        }

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        if !parser.errors.is_empty() {
            for error in parser.errors {
                eprintln!("Parser error: {}", error);
            }
            continue;
        }

        let evaluated = evaluator::eval(program, &mut env);
        // Don't print Null for statements that don't return a value
        if evaluated != object::Object::Null {
            println!("{}", evaluated);
        }
    }
}