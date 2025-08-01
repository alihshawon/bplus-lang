// compiler/src/environment.rs

use crate::object::Object;
use std::collections::HashMap;
use std::io::{self, Write};  // For real user input

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
                if args.len() > 1 {
                    return Object::Error(format!(
                        "wrong number of arguments. got={}, want=0 or 1",
                        args.len()
                    ));
                }

                let prompt = if args.len() == 1 {
                    format!("{}", args[0])
                } else {
                    "".to_string()
                };

                print!("{}", prompt);
                io::stdout().flush().unwrap();

                let mut input_line = String::new();
                match io::stdin().read_line(&mut input_line) {
                    Ok(_) => {
                        let input_str = input_line.trim().to_string();
                        Object::String(input_str)
                    }
                    Err(_) => Object::Error("Failed to read input".to_string()),
                }
            }),
        );

        // 'shomoy' builtin function placeholder, can return current time as string
        store.insert(
            "shomoy".to_string(),
            Object::BuiltinNative(|_args| {
                use chrono::Local;
                let now = Local::now();
                Object::String(now.format("%Y-%m-%d %H:%M:%S").to_string())
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
                println!("প্রোগ্রাম বন্ধ করা হলো। ধন্যবাদ!");
                Object::Null
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
}
