// compiler/src/main.rs

// Import all necessary modules for the compiler
mod ast;
mod environment;
mod evaluator;
mod lexer;
mod object;
mod parser;
mod token;
mod error;
#[path = "extension-manager.rs"]
mod extension_manager;

use environment::Environment;
use lexer::Lexer;
use parser::Parser;
use error::{BPlusError, ErrorType, ErrorManager};
use extension_manager::ExtensionManager;
use std::env;
use std::fs;
use std::io::{self, Write};

// Function to check if all curly brackets in input are balanced
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

// Function to run source code with error management and evaluation
fn run_source_with_error_manager(source: &str, error_manager: &ErrorManager) {
    let mut env = Environment::new();
    let lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    // Print parsing errors if any, then return
    if !parser.errors.is_empty() {
        for rust_error in parser.errors {
            let bp_error = BPlusError::new(ErrorType::InvalidStatement(rust_error));
            error_manager.print_error(&bp_error);
        }
        return;
    }

    // Evaluate parsed program and print result or error
    let evaluated = evaluator::eval(program, &mut env);
    if evaluated != object::Object::Null {
        match &evaluated {
            object::Object::Error(msg) => {
                let bp_error = BPlusError::new(ErrorType::InternalError(msg.clone()));
                error_manager.print_error(&bp_error);
            }
            _ => println!("{}", evaluated)
        }
    }
}

// Main entry point of the compiler/interpreter executable
fn main() {
    // Initialize the extension system to manage language packs
    let mut extension_manager = ExtensionManager::default();
    
    // Attempt to initialize extensions and print welcome messages
    match extension_manager.initialize() {
        Ok(()) => {
            if let Some(pack) = extension_manager.get_active_language_pack() {
                let welcome_default = format!("Active language pack: {} ({})", pack.language, pack.version);
                let welcome_msg = pack.keyword_mappings.get("welcome_message")
                    .unwrap_or(&welcome_default);
                println!("{}", welcome_msg);

                if let Some(example) = pack.keyword_mappings.get("example_usage") {
                    println!("{}", example);
                }
            } else {
                // Default welcome message in Banglish if no language pack active
                println!("B+ e Apnake Shagotom!");
                println!("Apni Phonetic Bangla keywords babohar korte parben.");
                println!("Cheshta korun: jodi (10 > 5) {{ dekhao(\"10 is greater than 5!\") }}");
            }
        }
        Err(e) => {
            // Handle extension initialization error and fallback message
            let error_default = "Extension system error".to_string();
            let fallback_default = "Default Banglish mode e cholche...".to_string();

            let error_msg = extension_manager.get_active_language_pack()
                .and_then(|pack| pack.keyword_mappings.get("extension_init_error"))
                .unwrap_or(&error_default);
            let fallback_msg = extension_manager.get_active_language_pack()
                .and_then(|pack| pack.keyword_mappings.get("fallback_mode"))
                .unwrap_or(&fallback_default);

            eprintln!("{}: {}", error_msg, e);
            eprintln!("{}", fallback_msg);
        }
    }

    let args: Vec<String> = env::args().collect();

    // File mode: run source code from file if filename provided as argument
    if args.len() > 1 {
        let filename = &args[1];
        match fs::read_to_string(filename) {
            Ok(source) => {
                run_source_with_error_manager(&source, extension_manager.get_error_manager());
            }
            Err(_) => {
                // Print file not found error
                let bp_error = BPlusError::new(ErrorType::FileNotFound(filename.clone()));
                extension_manager.get_error_manager().print_error(&bp_error);
            }
        }
        return;
    }

    // REPL (Read-Eval-Print Loop) mode initialization message
    let repl_default = "REPL mode shuru holo. 'prosthan' likhe ber hon.".to_string();
    let repl_start_msg = extension_manager.get_active_language_pack()
        .and_then(|pack| pack.keyword_mappings.get("repl_start"))
        .unwrap_or(&repl_default);
    println!("{}", repl_start_msg);

    let mut env = Environment::new();
    let mut input_buffer = String::new();

    // Start REPL loop to read input lines until exit command
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

        // Exit REPL on "prosthan" command
        if line.trim() == "prosthan" {
            break;
        }

        // Handle language pack activation commands inside REPL
        if line.trim().starts_with("langpack ") {
            let parts: Vec<&str> = line.trim().split_whitespace().collect();
            if parts.len() == 2 {
                let pack_name = parts[1];
                match extension_manager.activate_language_pack(pack_name) {
                    Ok(()) => println!("Language pack '{}' activate kora holo", pack_name),
                    Err(e) => println!("Language pack activate korte parini: {}", e),
                }
            } else {
                println!("Usage: langpack <name>");
                println!("Example: langpack english");
            }
            continue;
        }

        // List available language packs
        if line.trim() == "langpack list" {
            println!("Available language packs:");
            println!("- english");
            println!("- bangla-unicode");
            continue;
        }

        // Append user input line to buffer
        input_buffer.push_str(&line);

        // When brackets balanced, parse and evaluate the buffered input
        if brackets_balanced(&input_buffer) {
            let lexer = Lexer::new(input_buffer.clone());
            let mut parser = Parser::new(lexer);
            let program = parser.parse_program();

            // Handle parsing errors if any
            if !parser.errors.is_empty() {
                for rust_error in parser.errors {
                    let bp_error = BPlusError::new(ErrorType::InvalidStatement(rust_error));
                    extension_manager.get_error_manager().print_error(&bp_error);
                }
                input_buffer.clear();
                continue;
            }

            // Evaluate the program and print results or errors
            let evaluated = evaluator::eval(program, &mut env);
            if evaluated != object::Object::Null {
                match &evaluated {
                    object::Object::Error(msg) => {
                        let bp_error = BPlusError::new(ErrorType::InternalError(msg.clone()));
                        extension_manager.get_error_manager().print_error(&bp_error);
                    }
                    _ => println!("{}", evaluated)
                }
            }
            input_buffer.clear();
        }
    }

    // Print goodbye message based on active language pack or default
    if let Some(pack) = extension_manager.get_active_language_pack() {
        match pack.language.as_str() {
            "English" => println!("Goodbye! Thanks for using B+!"),
            "Bengali Unicode" => println!("বিদায়! বি+ ব্যবহার করার জন্য ধন্যবাদ!"),
            _ => println!("Dhonnobad! B+ bebhar korar jonno!")
        }
    } else {
        println!("Dhonnobad! B+ bebhar korar jonno!");
    }
}

// Unit tests for the main module
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_brackets_balanced() {
        // Test balanced brackets function for various inputs
        assert!(brackets_balanced("{ test }"));
        assert!(!brackets_balanced("{ test"));
        assert!(!brackets_balanced("test }"));
    }

    #[test]
    fn test_extension_manager() {
        // Test initialization of extension manager and default language
        let mut ext_manager = ExtensionManager::new("test_extensions");
        let error_manager = ext_manager.get_error_manager();
        assert_eq!(error_manager.get_current_language(), "banglish");
    }
}
