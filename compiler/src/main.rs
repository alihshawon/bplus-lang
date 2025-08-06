// compiler/src/main.rs

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

fn run_source_with_error_manager(source: &str, error_manager: &ErrorManager) {
    let mut env = Environment::new();
    let lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    if !parser.errors.is_empty() {
        for rust_error in parser.errors {
            let bp_error = BPlusError::new(ErrorType::InvalidStatement(rust_error));
            error_manager.print_error(&bp_error);
        }
        return;
    }

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

fn main() {
    // Initialize extension system
    let mut extension_manager = ExtensionManager::default();
    
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
                println!("B+ te apnake svagatam!");
                println!("Apni ekhon default Banglish keywords bebhar korte paren.");
                println!("Cheshta korun: jodi (10 > 5) {{ dekhao(\"10 is greater than 5!\") }}");
            }
        }
        Err(e) => {
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

    // File mode
    if args.len() > 1 {
        let filename = &args[1];
        match fs::read_to_string(filename) {
            Ok(source) => {
                run_source_with_error_manager(&source, extension_manager.get_error_manager());
            }
            Err(_) => {
                let bp_error = BPlusError::new(ErrorType::FileNotFound(filename.clone()));
                extension_manager.get_error_manager().print_error(&bp_error);
            }
        }
        return;
    }

    // REPL mode
    let repl_default = "REPL mode shuru holo. 'prosthan' likhe ber hon.".to_string();
    let repl_start_msg = extension_manager.get_active_language_pack()
        .and_then(|pack| pack.keyword_mappings.get("repl_start"))
        .unwrap_or(&repl_default);
    println!("{}", repl_start_msg);


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

        // Language pack management commands
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

        if line.trim() == "langpack list" {
            println!("Available language packs:");
            println!("- english");
            println!("- bangla-unicode");
            continue;
        }

        input_buffer.push_str(&line);

        if brackets_balanced(&input_buffer) {
            let lexer = Lexer::new(input_buffer.clone());
            let mut parser = Parser::new(lexer);
            let program = parser.parse_program();

            if !parser.errors.is_empty() {
                for rust_error in parser.errors {
                    let bp_error = BPlusError::new(ErrorType::InvalidStatement(rust_error));
                    extension_manager.get_error_manager().print_error(&bp_error);
                }
                input_buffer.clear();
                continue;
            }

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

    // Goodbye message
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_brackets_balanced() {
        assert!(brackets_balanced("{ test }"));
        assert!(!brackets_balanced("{ test"));
        assert!(!brackets_balanced("test }"));
    }

    #[test]
    fn test_extension_manager() {
        let mut ext_manager = ExtensionManager::new("test_extensions");
        let error_manager = ext_manager.get_error_manager();
        assert_eq!(error_manager.get_current_language(), "banglish");
    }
}