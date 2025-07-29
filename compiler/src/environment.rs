// compiler/src/environment.rs

//use std::io::{self, Write}; // For printing in built-in functions
use crate::object::Object;
use std::collections::HashMap;
use crate::evaluator::builtin_input;


// compiler/src/environment.rs
#[derive(Clone, Debug, PartialEq)]
pub struct Environment {
    store: HashMap<String, Object>,
    outer: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Environment {
        let mut store = HashMap::new();

        // --- Define Built-in Functions Here ---

        // Define the 'dekhao' function
        store.insert(
            "dekhao".to_string(),
            Object::Builtin(|args| {
                if args.len() != 1 {
                    return Object::Error(format!(
                        "wrong number of arguments. got={}, want=1",
                        args.len()
                    ));
                }
                // The actual print happens here
                println!("{}", args[0]);
                Object::Null
            }),
        );
    

        store.insert("input".to_string(), Object::Builtin(builtin_input));

    /*
        store.insert(
            "input".to_string(),
            Object::Builtin(|args| {
                if args.len() != 1 {
                    return Object::Error(format!(
                        "wrong number of arguments. got={}, want=1",
                        args.len()
                    ));
                }
                print!("{}", args[0]);  // prompt string print korbe
                io::stdout().flush().unwrap();

                let mut input = String::new();
                match io::stdin().read_line(&mut input) {
                    Ok(_) => Object::String(input.trim().to_string()),
                    Err(_) => Object::Error("failed to read input".to_string()),
                }
            }),
        );
    */








        // You can add more built-in functions here later
        // For example, a len() function:
        // store.insert("len".to_string(), Object::Builtin(...));

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
}
