// compiler/src/main.rs

mod lexer;
mod parser;
mod type_checker;
mod codegen;

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: bplusc <source_file.bp>");
        std::process::exit(1);
    }

    let filename = &args[1];
    let source = match fs::read_to_string(filename) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error reading file {}: {}", filename, err);
            std::process::exit(1);
        }
    };

    let tokens = lexer::tokenize(&source);
    let mut parser = parser::Parser::new(tokens);
    let ast = parser.parse_expression();

    let ty = type_checker::TypeChecker::check(&ast);
    if ty == type_checker::Type::Unknown {
        eprintln!("Type checking failed.");
        std::process::exit(1);
    }

    let output = codegen::CodeGenerator::generate(&ast);

    // For now, print the output to stdout
    println!("Generated Code:\n{}", output);
}