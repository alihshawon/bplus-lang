// compiler/src/environment.rs

// === IMPORTS ===
// Importing 'Object' type from object.rs file
use crate::object::Object;

// Using standard HashMap for variable bindings
use std::collections::HashMap;

// For handling user input and flushing output
use std::io::{self, Write};


// === ENVIRONMENT STRUCTURE ===
// The Environment holds variable and function bindings.
// It can have an optional outer environment (for nested scopes).
#[derive(Clone, Debug, PartialEq)]
pub struct Environment {
    store: HashMap<String, Object>,           // Variable/function storage
    outer: Option<Box<Environment>>,          // Optional parent environment (for closures, scopes)
}


// === ENVIRONMENT IMPLEMENTATION START ===
impl Environment {

    // === FUNCTION: new ===
    // Creates a new root environment with builtin functions preloaded
    pub fn new() -> Environment {
        let mut store = HashMap::new();

        // === BUILTIN: dekhao ===
        // A native print-like function that prints one argument
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

        // === BUILTIN: input ===
        // Asks the user for input with optional prompt message
        store.insert(
            "input".to_string(),
            Object::BuiltinNative(|args| {
                let prompt = if !args.is_empty() {
                    format!("{}", args[0])
                } else {
                    "".to_string()
                };

                // Debugging line (can be removed in production)
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


        // === BUILTIN: shomoy ===
        // Returns current time/date/timestamp in different formats
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


        // === BUILTIN: readkoro ===
        // Reads content of a file (filename must be provided)
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


        // === BUILTIN: writekoro ===
        // Writes string content into a file (filename + content required)
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


        // === BUILTIN: shuru_koro ===
        // Prints a restart message (does not actually restart the program)
        store.insert(
            "shuru_koro".to_string(),
            Object::BuiltinNative(|_args| {
                println!("প্রোগ্রাম পুনরায় শুরু হচ্ছে...");
                Object::Null
            }),
        );


        // === BUILTIN: bondho_koro ===
        // Prints a shutdown message (does not actually terminate program)
        store.insert(
            "bondho_koro".to_string(),
            Object::BuiltinNative(|_args| {
                println!("Program bondho kora holo. Dhonnobad!");
                Object::Null
            }),
        );


        // === BUILTIN: exitkoro ===
        // Terminates the program with optional exit code
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

        // Return the final environment with all built-ins loaded
        Environment { store, outer: None }
    }


    // === FUNCTION: new_enclosed ===
    // Creates a new inner (child) environment with a parent scope
    pub fn new_enclosed(outer: Environment) -> Environment {
        Environment {
            store: HashMap::new(),
            outer: Some(Box::new(outer)),
        }
    }


    // === FUNCTION: get ===
    // Retrieves a value by name from the current or outer environment
    pub fn get(&self, name: &str) -> Option<Object> {
        match self.store.get(name) {
            Some(obj) => Some(obj.clone()),
            None => self.outer.as_ref().and_then(|o| o.get(name)),
        }
    }


    // === FUNCTION: set ===
    // Sets a variable in the current environment
    pub fn set(&mut self, name: String, val: Object) -> Object {
        self.store.insert(name, val.clone());
        val
    }


    // === FUNCTION: has_builtin ===
    // Checks whether a builtin or variable exists in the current environment
    pub fn has_builtin(&self, name: &str) -> bool {
        self.store.contains_key(name)
    }


    // === FUNCTION: add_builtin ===
    // Manually adds a new builtin function to the environment
    pub fn add_builtin(&mut self, name: String, func: Object) {
        self.store.insert(name, func);
    }
}
// === ENVIRONMENT IMPLEMENTATION END ===
