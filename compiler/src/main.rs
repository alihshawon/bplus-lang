// compiler/src/main.rs

// Import all necessary modules for the compiler
mod stdlib;
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
use std::path::Path;

use log::{error, info, warn};

/// Function to check if all curly brackets in input are balanced
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

/// Function to run source code with error management and evaluation
fn run_source_with_error_manager(source: &str, error_manager: &ErrorManager) -> Result<(), ()> {
    // Create a new environment for the program execution
    let mut env = Environment::new();
    // Initialize lexer with source code
    let lexer = Lexer::new(source.to_string());
    // Create parser from lexer
    let mut parser = Parser::new(lexer);
    // Parse the entire program into AST
    let program = parser.parse_program();

    // If parser has errors, print them and return error
    if !parser.errors.is_empty() {
        for rust_error in parser.errors {
            let bp_error = BPlusError::new(ErrorType::InvalidStatement(rust_error));
            error_manager.print_error(&bp_error);
        }
        return Err(());
    }

    // Evaluate the parsed program and print result or errors
    let evaluated = evaluator::eval(program, &mut env);
    if evaluated != object::Object::Null {
        match &evaluated {
            object::Object::Error(msg) => {
                let bp_error = BPlusError::new(ErrorType::InternalError(msg.clone()));
                error_manager.print_error(&bp_error);
                return Err(());
            }
            _ => println!("{}", evaluated),
        }
    }
    Ok(())
}

/// Initialize logging for the compiler using env_logger
fn init_logging() {
    env_logger::init();
}

/// Main entry point of the compiler/interpreter executable
fn main() {
    // Initialize logging system
    init_logging();

    info!("Starting B+ compiler/interpreter...");

    // Initialize the extension system to manage language packs
    let mut extension_manager = ExtensionManager::default();

    // Attempt to initialize extensions and print welcome messages
    match extension_manager.initialize() {
        Ok(()) => {
            // If active language pack present, print welcome message and example usage
            if let Some(pack) = extension_manager.get_active_language_pack() {
                let welcome_default = format!("Active language pack: {} ({})", pack.language, pack.version);
                let welcome_msg = pack.keyword_mappings.get("welcome_message").unwrap_or(&welcome_default);
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

            let error_msg = extension_manager
                .get_active_language_pack()
                .and_then(|pack| pack.keyword_mappings.get("extension_init_error"))
                .unwrap_or(&error_default);

            let fallback_msg = extension_manager
                .get_active_language_pack()
                .and_then(|pack| pack.keyword_mappings.get("fallback_mode"))
                .unwrap_or(&fallback_default);

            error!("{}: {}", error_msg, e);
            eprintln!("{}: {}", error_msg, e);
            eprintln!("{}", fallback_msg);
        }
    }

    // Collect command line arguments
    let args: Vec<String> = env::args().collect();

    // If filename argument provided, run the file and exit
    if args.len() > 1 {
        let filename = &args[1];
        let path = Path::new(filename);

        match fs::read_to_string(path) {
            Ok(source) => {
                if let Err(_) = run_source_with_error_manager(&source, extension_manager.get_error_manager()) {
                    error!("Error occurred while running source file: {}", filename);
                }
            }
            Err(e) => {
                // File read error handling
                let bp_error = BPlusError::new(ErrorType::FileNotFound(filename.clone()));
                extension_manager.get_error_manager().print_error(&bp_error);
                error!("Failed to read file '{}': {}", filename, e);
            }
        }
        return;
    }

    // REPL mode welcome message
    let repl_default = "REPL mode shuru holo. 'prosthan' likhe ber hon.".to_string();
    let repl_start_msg = extension_manager
        .get_active_language_pack()
        .and_then(|pack| pack.keyword_mappings.get("repl_start"))
        .unwrap_or(&repl_default);

    println!("{}", repl_start_msg);

    // Initialize environment for REPL
    let mut env = Environment::new();
    let mut input_buffer = String::new();

    // Start REPL loop to read input lines until exit command
    loop {
        // Print prompt based on buffer state
        if input_buffer.is_empty() {
            print!(">> ");
        } else {
            print!("... ");
        }

        // Flush stdout and check for errors
        if io::stdout().flush().is_err() {
            error!("Failed to flush stdout");
            break;
        }

        let mut line = String::new();
        let read_res = io::stdin().read_line(&mut line);

        // Handle stdin reading errors or EOF (Ctrl-D)
        if let Err(e) = read_res {
            error!("Error reading stdin: {}", e);
            break;
        }
        if read_res.unwrap() == 0 {
            // EOF detected - exit gracefully with goodbye message
            println!("\n{}", extension_manager.get_active_language_pack()
                .and_then(|pack| pack.keyword_mappings.get("goodbye"))
                .unwrap_or(&"Goodbye!".to_string()));
            break;
        }

        let trimmed_line = line.trim();

        // Exit REPL on 'prosthan' command
        if trimmed_line == "prosthan" {
            break;
        }

        // Handle import command inside REPL: anyo or import
        if trimmed_line.starts_with("anyo ") || trimmed_line.starts_with("import ") {
            let parts: Vec<&str> = trimmed_line.split_whitespace().collect();
            if parts.len() >= 2 {
                let module_name = parts[1];
                match crate::stdlib::load_stdlib_module(&mut env, module_name) {
                    Ok(()) => {
                        info!("Module '{}' loaded successfully", module_name);
                    }
                    Err(e) => println!("Import error: {}", e),
                }
            } else {
                println!("Usage: anyo <module_name>");
                println!("Available modules: {}", stdlib::get_available_modules().join(", "));
            }
            continue;
        }

        // List available modules command
        if trimmed_line == "modules" || trimmed_line == "module list" {
            println!("Available modules:");
            for module in stdlib::get_available_modules() {
                println!("  - {}", module);
            }
            continue;
        }

        // Language pack activation command inside REPL
        if trimmed_line.starts_with("langpack ") {
            let parts: Vec<&str> = trimmed_line.split_whitespace().collect();
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
        if trimmed_line == "langpack list" {
            println!("Available language packs:");
            println!("- english");
            println!("- bangla-unicode");
            continue;
        }

        // Append current input line to buffer
        input_buffer.push_str(&line);

        // Parse and evaluate when brackets balanced
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

            // Evaluate program and print results or errors
            let evaluated = evaluator::eval(program, &mut env);
            if evaluated != object::Object::Null {
                match &evaluated {
                    object::Object::Error(msg) => {
                        let bp_error = BPlusError::new(ErrorType::InternalError(msg.clone()));
                        extension_manager.get_error_manager().print_error(&bp_error);
                    }
                    _ => println!("{}", evaluated),
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
            _ => println!("Dhonnobad! B+ bebhar korar jonno!"),
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
    fn test_extension_manager_language() {
        // Test initialization of extension manager and default language
        let mut ext_manager = ExtensionManager::new("test_extensions");
        let error_manager = ext_manager.get_error_manager();
        assert_eq!(error_manager.get_current_language(), "banglish");
    }
}
