// compiler/src/stdlib/time.rs

use crate::environment::Environment;
use crate::object::Object;

/// Load all time-related functions into environment
pub fn load_time_functions(env: &mut Environment) {
    // Move shomoy function from environment.rs to here
    env.add_builtin("shomoy".to_string(), Object::BuiltinNative(shomoy_function));
    env.add_builtin("time".to_string(), Object::BuiltinNative(shomoy_function));
    
    // Add new time functions
    env.add_builtin("timestamp".to_string(), Object::BuiltinNative(timestamp_function));
    env.add_builtin("date".to_string(), Object::BuiltinNative(date_function));
    env.add_builtin("sleep".to_string(), Object::BuiltinNative(sleep_function));
}

/// Original shomoy function
fn shomoy_function(args: Vec<Object>) -> Object {
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
}

/// Get current timestamp
fn timestamp_function(_args: Vec<Object>) -> Object {
    use chrono::Local;
    Object::Integer(Local::now().timestamp())
}

/// Get current date only
fn date_function(_args: Vec<Object>) -> Object {
    use chrono::Local;
    Object::String(Local::now().format("%Y-%m-%d").to_string())
}

/// Sleep for specified seconds
fn sleep_function(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return Object::Error("sleep() requires exactly one argument (seconds)".to_string());
    }
    
    match &args[0] {
        Object::Integer(seconds) => {
            std::thread::sleep(std::time::Duration::from_secs(*seconds as u64));
            Object::Null
        }
        _ => Object::Error("sleep() requires an integer argument".to_string()),
    }
}