// compiler/src/environment.rs

use crate::object::Object;
use std::collections::HashMap;
use std::io::{self, Write}; // For real user input

#[derive(Clone, Debug, PartialEq)]
pub struct Environment {
    store: HashMap<String, Object>,
    outer: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Environment {
        let mut store = HashMap::new();

        // 'dekhao' builtin function
        store.insert(
            "dekhao".to_string(),
            Object::BuiltinNative(|args| {
                if args.len() != 1 {
                    return Object::Error(format!(
                        "wrong number of arguments. got={}, want=1",
                        args.len()
                    ));
                }
                println!("{}", args[0]);
                Object::Null
            }),
        );

        // 'input' builtin function with real input reading
        store.insert(
            "input".to_string(),
            Object::BuiltinNative(|args| {
                let prompt = if !args.is_empty() {
                    format!("{}", args[0])
                } else {
                    "".to_string()
                };

                // Debug line - remove after testing
                println!("DEBUG: input prompt to print: '{}'", prompt);

                print!("{}", prompt);
                io::stdout().flush().unwrap();

                let mut input_line = String::new();
                match io::stdin().read_line(&mut input_line) {
                    Ok(_) => Object::String(input_line.trim().to_string()),
                    Err(e) => Object::Error(format!("Input error: {}", e)),
                }
            }),
        );


        // 'shomoy' builtin function with better formatting
        store.insert(
            "shomoy".to_string(),
            Object::BuiltinNative(|args| {
                use chrono::Local;
                let now = Local::now();

                if !args.is_empty() {
                    match args[0] {
                        Object::String(ref format_str) => match format_str.as_str() {
                            "timestamp" => Object::Integer(now.timestamp()),
                            "date" => Object::String(now.format("%Y-%m-%d").to_string()),
                            "time" => Object::String(now.format("%H:%M:%S").to_string()),
                            _ => Object::String(now.format("%Y-%m-%d %H:%M:%S").to_string()),
                        },
                        _ => Object::String(now.format("%Y-%m-%d %H:%M:%S").to_string()),
                    }
                } else {
                    Object::String(now.format("%Y-%m-%d %H:%M:%S").to_string())
                }
            }),
        );

        // File reading function
        store.insert(
            "readkoro".to_string(),
            Object::BuiltinNative(|args| {
                if args.len() != 1 {
                    return Object::Error("readkoro() requires exactly one argument (filename)".to_string());
                }

                match &args[0] {
                    Object::String(filename) => match std::fs::read_to_string(filename) {
                        Ok(content) => Object::String(content),
                        Err(e) => Object::Error(format!("File read error: {}", e)),
                    },
                    _ => Object::Error("readkoro() requires a string filename".to_string()),
                }
            }),
        );

        // File writing function
        store.insert(
            "writekoro".to_string(),
            Object::BuiltinNative(|args| {
                if args.len() != 2 {
                    return Object::Error("writekoro() requires exactly two arguments (filename, content)".to_string());
                }

                match (&args[0], &args[1]) {
                    (Object::String(filename), content) => {
                        let content_str = format!("{}", content);
                        match std::fs::write(filename, content_str) {
                            Ok(_) => Object::Null,
                            Err(e) => Object::Error(format!("File write error: {}", e)),
                        }
                    }
                    _ => Object::Error("writekoro() requires a string filename as first argument".to_string()),
                }
            }),
        );

        // 'shuru_koro' builtin function
        store.insert(
            "shuru_koro".to_string(),
            Object::BuiltinNative(|_args| {
                println!("প্রোগ্রাম পুনরায় শুরু হচ্ছে...");
                Object::Null
            }),
        );

        // 'bondho_koro' builtin function
        store.insert(
            "bondho_koro".to_string(),
            Object::BuiltinNative(|_args| {
                println!("Program bondho kora holo. Dhonnobad!");
                Object::Null
            }),
        );

        // exitkoro builtin function
        store.insert(
            "exitkoro".to_string(),
            Object::BuiltinNative(|args| {
                let exit_code = if !args.is_empty() {
                    match &args[0] {
                        Object::Integer(code) => *code as i32,
                        _ => 0,
                    }
                } else {
                    0
                };

                println!("Program theke exit kora hosse!");
                std::process::exit(exit_code);
            }),
        );

        Environment { store, outer: None }
    }

    pub fn new_enclosed(outer: Environment) -> Environment {
        Environment {
            store: HashMap::new(),
            outer: Some(Box::new(outer)),
        }
    }

    pub fn get(&self, name: &str) -> Option<Object> {
        match self.store.get(name) {
            Some(obj) => Some(obj.clone()),
            None => self.outer.as_ref().and_then(|o| o.get(name)),
        }
    }

    pub fn set(&mut self, name: String, val: Object) -> Object {
        self.store.insert(name, val.clone());
        val
    }

    pub fn has_builtin(&self, name: &str) -> bool {
        self.store.contains_key(name)
    }

    pub fn add_builtin(&mut self, name: String, func: Object) {
        self.store.insert(name, func);
    }
}
