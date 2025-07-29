// compiler/src/main.rs

// Removed unused module declarations
// compiler/src/main.rs

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

fn brackets_balanced(input: &str) -> bool {
    let mut count = 0;
    for c in input.chars() {
        if c == '{' {
            count += 1;
        } else if c == '}' {
            if count == 0 {
                return false;
            }
            count -= 1;
        }
    }
    count == 0
}

fn main() {
    println!("Welcome to the B+!");
    println!("You can now use Bangla keywords.");
    println!("Try koro: jodi (10 > 5) {{ dekhao(\"10 is greater than 5!\") }}");

    let mut env = Environment::new();
    let mut input_buffer = String::new();

    loop {
        if input_buffer.is_empty() {
            print!(">> ");
        } else {
            print!("... ");
        }
        io::stdout().flush().unwrap();

        let mut line = String::new();
        if io::stdin().read_line(&mut line).is_err() {
            break;
        }

        if line.trim() == "prosthan" {
            break;
        }

        input_buffer.push_str(&line);

        if brackets_balanced(&input_buffer) {
            let lexer = Lexer::new(input_buffer.clone());
            let mut parser = Parser::new(lexer);
            let program = parser.parse_program();

            if !parser.errors.is_empty() {
                for error in parser.errors {
                    eprintln!("Parser error: {}", error);
                }
                input_buffer.clear();
                continue;
            }

            let evaluated = evaluator::eval(program, &mut env);
            if evaluated != object::Object::Null {
                println!("{}", evaluated);
            }
            input_buffer.clear();
        }
        // else: continue reading lines until balanced
    }
}
// End of main.rs