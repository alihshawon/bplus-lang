// compiler/src/environment.rs

// === IMPORTS ===
// Importing 'Object' type from object.rs file
use crate::object::Object;

// Using standard HashMap for variable bindings
use std::collections::HashMap;

// For handling user input and flushing output
use std::io::{self, Write};

// === VARIABLE STRUCT ===
#[derive(Clone, Debug, PartialEq)]
pub struct Variable {
    pub value: Object,
    pub mutable: bool,
}

// === ENVIRONMENT STRUCTURE ===
// The Environment holds variable and function bindings.
// It can have an optional outer environment (for nested scopes).
#[derive(Clone, Debug, PartialEq)]
pub struct Environment {
    store: HashMap<String, Variable>,           // Variable/function storage
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
            Variable {
                value: Object::BuiltinNative(|args| {
                    if args.len() != 1 {
                        return Object::Error(format!(
                            "wrong number of arguments. got={}, want=1",
                            args.len()
                        ));
                    }
                    println!("{}", args[0]);
                    Object::Null
                }),
                mutable: true,
            },
        );

        // === BUILTIN: input ===
        // Asks the user for input with optional prompt message
        store.insert(
            "input".to_string(),
            Variable {
                value: Object::BuiltinNative(|args| {
                    let prompt = if !args.is_empty() {
                        format!("{}", args[0])
                    } else {
                        "".to_string()
                    };

                    print!("{}", prompt);
                    io::stdout().flush().unwrap();

                    let mut input_line = String::new();
                    match io::stdin().read_line(&mut input_line) {
                        Ok(_) => Object::String(input_line.trim().to_string()),
                        Err(e) => Object::Error(format!("Input error: {}", e)),
                    }
                }),
                mutable: true,
            },
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
            Some(var) => Some(var.value.clone()),
            None => self.outer.as_ref().and_then(|o| o.get(name)),
        }
    }

    // === FUNCTION: set ===
    // Sets a variable in the current environment
    pub fn set(&mut self, name: String, val: Object, mutable: bool) -> Object {
        self.store.insert(name, Variable { value: val.clone(), mutable });
        val
    }

    pub fn assign(&mut self, name: String, value: Object) -> Result<(), String> {
        if let Some(var) = self.store.get_mut(&name) {
            if var.mutable {
                var.value = value;
                Ok(())
            } else {
                Err(format!("Cannot assign to immutable variable '{}'", name))
            }
        } else {
            // Auto-declare on first assignment as immutable by default
            self.store.insert(name, Variable { value, mutable: false });
            Ok(())
        }
    }


    // === FUNCTION: has_builtin ===
    // Checks whether a builtin or variable exists in the current environment
    pub fn has_builtin(&self, name: &str) -> bool {
        self.store.contains_key(name)
    }

    // === FUNCTION: add_builtin ===
    // Manually adds a new builtin function to the environment
    pub fn add_builtin(&mut self, name: String, func: Object) {
        self.store.insert(name, Variable { value: func, mutable: true });
    }
}
// === ENVIRONMENT IMPLEMENTATION END ===

