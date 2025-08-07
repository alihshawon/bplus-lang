// compiler/src/stdlib/mod.rs

// This is the main module that manages all stdlib modules

// Declare sub-modules
pub mod time;
pub mod file;
pub mod system;
pub mod math;
pub mod string;

use crate::environment::Environment;

/// Load a standard library module into the environment
pub fn load_stdlib_module(env: &mut Environment, module_name: &str) -> Result<(), String> {
    match module_name {
        // Time module variants
        "time" | "shomoy" | "somoy" => {
            time::load_time_functions(env);
            println!("Time module loaded successfully");
            Ok(())
        }
        
        // File module variants  
        "file" | "faile" => {
            file::load_file_functions(env);
            println!("File module loaded successfully");
            Ok(())
        }
        
        // System module variants
        "system" | "sistam" => {
            system::load_system_functions(env);
            println!("System module loaded successfully");
            Ok(())
        }
        
        // Math module variants
        "math" | "gonit" => {
            math::load_math_functions(env);
            println!("Math module loaded successfully");
            Ok(())
        }
        
        // String module variants
        "string" | "shobdo" => {
            string::load_string_functions(env);
            println!("String module loaded successfully");
            Ok(())
        }
        
        _ => Err(format!("Unknown module: '{}'. Available modules: time, file, system, math, string", module_name))
    }
}

/// Get list of available modules
pub fn get_available_modules() -> Vec<&'static str> {
    vec!["time", "file", "system", "math", "string"]
}

/// Load commonly used modules automatically  
pub fn load_default_modules(env: &mut Environment) {
    // Optionally auto-load commonly used modules
    let _ = load_stdlib_module(env, "time");
    let _ = load_stdlib_module(env, "math");
}