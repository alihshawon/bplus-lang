// compiler/src/environment.rs

use crate::object::Object;
use std::collections::HashMap;

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
