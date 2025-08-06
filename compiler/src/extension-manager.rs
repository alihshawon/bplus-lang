// compiler/src/extension-manager.rs

use crate::error::{ErrorManager, LanguagePack};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub struct ExtensionManager {
    language_packs: HashMap<String, LanguagePack>,
    active_language_pack: Option<String>,
    extensions_path: String,
    error_manager: ErrorManager,
}

impl ExtensionManager {
    pub fn new(extensions_path: &str) -> Self {
        ExtensionManager {
            language_packs: HashMap::new(),
            active_language_pack: None,
            extensions_path: extensions_path.to_string(),
            error_manager: ErrorManager::new(), // Default Bangla
        }
    }
    
    pub fn initialize(&mut self) -> Result<(), String> {
        // 1. Check if extensions directory exists
        if !Path::new(&self.extensions_path).exists() {
            // Create extensions directory structure
            self.create_extension_directories()?;
        }
        
        // 2. Load language packs
        self.load_language_packs()?;
        
        // 3. Load configuration and set active language pack
        self.load_extension_config()?;
        
        Ok(())
    }
    
    fn create_extension_directories(&self) -> Result<(), String> {
        let base_path = Path::new(&self.extensions_path);
        
        // Create directory structure
        let dirs = [
            "language-packs",
            "runtime-extensions",
            "compiler-plugins",
        ];
        
        for dir in &dirs {
            let dir_path = base_path.join(dir);
            fs::create_dir_all(&dir_path)
                .map_err(|e| format!("Failed to create directory {:?}: {}", dir_path, e))?;
        }
        
        // Create default extensions.config
        let config_path = base_path.join("extensions.config");
        if !config_path.exists() {
            let default_config = r#"# B+ Language Extensions Configuration
# This file controls which extensions are loaded and enabled

[general]
auto_load = true
compatibility_check = true

[language_packs]
# Available language packs (set enabled = true to activate)
english = { enabled = false, version = "1.0" }
hindi = { enabled = false, version = "1.0" }

[runtime_extensions]
# Runtime extensions (loaded at runtime)
auto_typecast = { enabled = false, priority = 1 }

[compiler_plugins]
# Compiler plugins (loaded at compile time)
# advanced_optimizer = { enabled = false, version = "1.0" }
"#;
            fs::write(&config_path, default_config)
                .map_err(|e| format!("Failed to create config file: {}", e))?;
        }
        
        Ok(())
    }
    
    fn load_language_packs(&mut self) -> Result<(), String> {
        let packs_dir = Path::new(&self.extensions_path).join("language-packs");
        
        if packs_dir.exists() {
            for entry in fs::read_dir(&packs_dir)
                .map_err(|e| format!("Failed to read language-packs directory: {}", e))? 
            {
                let entry = entry.map_err(|e| format!("Error reading directory entry: {}", e))?;
                let path = entry.path();
                
                if let Some(extension) = path.extension() {
                    if extension == "bplp" {
                        // Load compiled language pack
                        if let Some(name) = path.file_stem().and_then(|s| s.to_str()) {
                            match self.load_compiled_language_pack(&path) {
                                Ok(pack) => {
                                    self.language_packs.insert(name.to_string(), pack);
                                    println!("Loaded language pack: {}", name);
                                }
                                Err(e) => {
                                    eprintln!("Failed to load language pack {}: {}", name, e);
                                }
                            }
                        }
                    } else if extension == "bplpsrc" {
                        // Compile source and load
                        if let Some(name) = path.file_stem().and_then(|s| s.to_str()) {
                            match self.compile_and_load_language_pack(&path) {
                                Ok(pack) => {
                                    self.language_packs.insert(name.to_string(), pack);
                                    println!("Compiled and loaded language pack: {}", name);
                                }
                                Err(e) => {
                                    eprintln!("Failed to compile language pack {}: {}", name, e);
                                }
                            }
                        }
                    }
                }
            }
        }
        
        Ok(())
    }
    
    fn load_compiled_language_pack(&self, path: &Path) -> Result<LanguagePack, String> {
        // For now, we'll implement a simple text-based format
        // In production, this would be a proper binary format
        let content = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read language pack file: {}", e))?;
        
        if content.starts_with("// Compiled Binery File for B Plus Language") {
            // This is a placeholder compiled file
            // For now, we'll use English as default for .bplp files
            Ok(self.create_english_language_pack())
        } else {
            Err("Invalid language pack format".to_string())
        }
    }
    
    fn compile_and_load_language_pack(&self, path: &Path) -> Result<LanguagePack, String> {
        let content = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read language pack source: {}", e))?;
        
        self.parse_language_pack_source(&content)
    }
    
    fn parse_language_pack_source(&self, content: &str) -> Result<LanguagePack, String> {
        let mut language = String::new();
        let mut version = String::new();
        let mut author = String::new();
        let mut keyword_mappings = HashMap::new();
        let mut error_templates = HashMap::new();
        
        let mut current_section = String::new();
        
        for line in content.lines() {
            let line = line.trim();
            
            // Skip comments and empty lines
            if line.starts_with('#') || line.is_empty() {
                continue;
            }
            
            // Section headers
            if line.starts_with('[') && line.ends_with(']') {
                current_section = line[1..line.len()-1].to_string();
                continue;
            }
            
            // Parse key-value pairs
            if let Some(eq_pos) = line.find('=') {
                let key = line[..eq_pos].trim();
                let value = line[eq_pos+1..].trim();
                
                match current_section.as_str() {
                    "metadata" => {
                        match key {
                            "language" => language = value.to_string(),
                            "version" => version = value.to_string(),
                            "author" => author = value.to_string(),
                            _ => {}
                        }
                    }
                    "mapping" => {
                        // Parse keyword mappings like "jodi => if"
                        if let Some(arrow_pos) = value.find("=>") {
                            let from_key = value[..arrow_pos].trim().to_string();
                            let to_key = value[arrow_pos+2..].trim().to_string();
                            keyword_mappings.insert(from_key, to_key);
                        }
                    }
                    "error_messages" => {
                        // Parse error message templates
                        error_templates.insert(key.to_string(), value.to_string());
                    }
                    _ => {}
                }
            }
        }
        
        // Add default English error messages if not provided in source
        if error_templates.is_empty() {
            error_templates = self.get_english_error_templates();
        }
        
        Ok(LanguagePack {
            language,
            version,
            author,
            keyword_mappings,
            error_templates,
        })
    }
    
    fn get_english_error_templates(&self) -> HashMap<String, String> {
        let mut templates = HashMap::new();
        
        templates.insert("unexpected_character".to_string(), 
            "Unexpected character '{0}' found".to_string());
        templates.insert("unterminated_string".to_string(), 
            "Unterminated string - missing quote mark".to_string());
        templates.insert("type_mismatch".to_string(), 
            "Type mismatch - expected '{0}' but got '{1}'".to_string());
        templates.insert("undefined_variable".to_string(), 
            "Undefined variable '{0}' - declare it first".to_string());
        templates.insert("division_by_zero".to_string(), 
            "Cannot divide by zero".to_string());
        // Add more templates as needed...
        
        templates
    }
    
    
    fn load_extension_config(&mut self) -> Result<(), String> {
        let config_path = Path::new(&self.extensions_path).join("extensions.config");
        
        if !config_path.exists() {
            return Ok(()); // No config, use defaults
        }
        
        let content = fs::read_to_string(&config_path)
            .map_err(|e| format!("Failed to read extensions.config: {}", e))?;
        
        // Parse TOML-like config (simplified parsing)
        let mut in_language_packs = false;
        
        for line in content.lines() {
            let line = line.trim();
            
            if line.starts_with('[') {
                in_language_packs = line == "[language_packs]";
                continue;
            }
            
            if in_language_packs && line.contains("enabled = true") {
                // Extract language pack name
                if let Some(eq_pos) = line.find('=') {
                    let pack_name = line[..eq_pos].trim();
                    if self.language_packs.contains_key(pack_name) {
                        self.activate_language_pack(pack_name)?;
                        break; // Only one language pack can be active
                    }
                }
            }
        }
        
        Ok(())
    }
    
    pub fn activate_language_pack(&mut self, pack_name: &str) -> Result<(), String> {
        if let Some(pack) = self.language_packs.get(pack_name) {
            self.active_language_pack = Some(pack_name.to_string());
            self.error_manager = ErrorManager::with_language_pack(pack);
            println!("Activated language pack: {}", pack_name);
            Ok(())
        } else {
            Err(format!("Language pack '{}' not found", pack_name))
        }
    }
    
    pub fn get_error_manager(&self) -> &ErrorManager {
        &self.error_manager
    }
    
    pub fn get_active_language_pack(&self) -> Option<&LanguagePack> {
        if let Some(ref pack_name) = self.active_language_pack {
            self.language_packs.get(pack_name)
        } else {
            None
        }
    }
    
    pub fn translate_keyword(&self, keyword: &str) -> String {
        if let Some(pack) = self.get_active_language_pack() {
            // Check if there's a mapping for this keyword
            if let Some(translated) = pack.keyword_mappings.get(keyword) {
                return translated.clone();
            }
            // Also check reverse mapping (for user input)
            for (bangla, english) in &pack.keyword_mappings {
                if english == keyword {
                    return bangla.clone();
                }
            }
        }
        keyword.to_string() // Return original if no translation found
    }
    
    pub fn is_valid_keyword(&self, keyword: &str) -> bool {
        // Check if keyword is valid in current language context
        if let Some(pack) = self.get_active_language_pack() {
            pack.keyword_mappings.contains_key(keyword) || 
            pack.keyword_mappings.values().any(|v| v == keyword)
        } else {
            // Default Bangla keywords
            matches!(keyword, 
                "dhoro" | "kaj" | "fn" | "ha" | "na" | "jodi" | "tahole" | 
                "nahoy" | "dekhao" | "input" | "ferot" | "shomoy" | "thamo"
            )
        }
    }


fn create_english_language_pack(&self) -> LanguagePack {
        let mut keyword_mappings = HashMap::new();
        
        // Keyword translations
        keyword_mappings.insert("jodi".to_string(), "if".to_string());
        keyword_mappings.insert("tahole".to_string(), "then".to_string());
        keyword_mappings.insert("nahoy".to_string(), "else".to_string());
        keyword_mappings.insert("dhoro".to_string(), "let".to_string());
        keyword_mappings.insert("kaj".to_string(), "function".to_string());
        keyword_mappings.insert("dekhao".to_string(), "print".to_string());
        keyword_mappings.insert("ferot".to_string(), "return".to_string());
        keyword_mappings.insert("ha".to_string(), "true".to_string());
        keyword_mappings.insert("na".to_string(), "false".to_string());
        
        // UI Messages in English
        keyword_mappings.insert("welcome_message".to_string(), 
            "Welcome to B+! English language pack is active.".to_string());
        keyword_mappings.insert("example_usage".to_string(), 
            "Try: if (10 > 5) { print(\"10 is greater than 5!\") }".to_string());
        keyword_mappings.insert("extension_init_error".to_string(), 
            "Extension system initialization failed".to_string());
        keyword_mappings.insert("fallback_mode".to_string(), 
            "Running in default Banglish mode...".to_string());
        keyword_mappings.insert("repl_start".to_string(), 
            "REPL mode started. Type 'exit' to quit.".to_string());
        keyword_mappings.insert("langpack_activated".to_string(), 
            "Language pack '{0}' has been activated".to_string());
        keyword_mappings.insert("langpack_error".to_string(), 
            "Failed to activate language pack: {0}".to_string());
        keyword_mappings.insert("langpack_usage".to_string(), 
            "Usage: langpack <name>\nExample: langpack english".to_string());
        keyword_mappings.insert("available_packs".to_string(), 
            "Available language packs:".to_string());
        keyword_mappings.insert("goodbye_message".to_string(), 
            "Goodbye! Thanks for using B+!".to_string());
        
        LanguagePack {
            language: "English".to_string(),
            version: "1.0".to_string(),
            author: "B+ Language Team".to_string(),
            keyword_mappings,
            error_templates: self.get_english_error_templates(),
        }
    }
    
    fn create_default_banglish_pack(&self) -> LanguagePack {
        let mut keyword_mappings = HashMap::new();
        
        // Default Banglish keywords (no translation needed)
        keyword_mappings.insert("jodi".to_string(), "jodi".to_string());
        keyword_mappings.insert("tahole".to_string(), "tahole".to_string());
        keyword_mappings.insert("nahoy".to_string(), "nahoy".to_string());
        keyword_mappings.insert("dhoro".to_string(), "dhoro".to_string());
        keyword_mappings.insert("kaj".to_string(), "kaj".to_string());
        keyword_mappings.insert("dekhao".to_string(), "dekhao".to_string());
        keyword_mappings.insert("ferot".to_string(), "ferot".to_string());
        keyword_mappings.insert("ha".to_string(), "ha".to_string());
        keyword_mappings.insert("na".to_string(), "na".to_string());
        
        // UI Messages in Banglish
        keyword_mappings.insert("welcome_message".to_string(), 
            "B+ te apnake svagatam!".to_string());
        keyword_mappings.insert("example_usage".to_string(), 
            "Cheshta korun: jodi (10 > 5) { dekhao(\"10 is greater than 5!\") }".to_string());
        keyword_mappings.insert("extension_init_error".to_string(), 
            "Extension system shuru korte problem".to_string());
        keyword_mappings.insert("fallback_mode".to_string(), 
            "Default Banglish mode e cholche...".to_string());
        keyword_mappings.insert("repl_start".to_string(), 
            "REPL mode shuru holo. 'prosthan' likhe ber hon.".to_string());
        keyword_mappings.insert("langpack_activated".to_string(), 
            "Language pack '{0}' activate kora holo".to_string());
        keyword_mappings.insert("langpack_error".to_string(), 
            "Language pack activate korte parini: {0}".to_string());
        keyword_mappings.insert("langpack_usage".to_string(), 
            "Usage: langpack <naam>\nExample: langpack english".to_string());
        keyword_mappings.insert("available_packs".to_string(), 
            "Available language packs:".to_string());
        keyword_mappings.insert("goodbye_message".to_string(), 
            "Dhonnobad! B+ bebhar korar jonno!".to_string());
        
        LanguagePack {
            language: "Banglish".to_string(),
            version: "1.0".to_string(),
            author: "B+ Language Team".to_string(),
            keyword_mappings,
            error_templates: HashMap::new(), // Uses default error manager
        }
    }
    
    pub fn get_message(&self, key: &str) -> String {
        if let Some(pack) = self.get_active_language_pack() {
            pack.keyword_mappings.get(key).cloned()
        } else {
            // Return default Banglish messages
            let default_pack = self.create_default_banglish_pack();
            default_pack.keyword_mappings.get(key).cloned()
        }.unwrap_or_else(|| format!("Missing message key: {}", key))
    }


}

impl Default for ExtensionManager {
    fn default() -> Self {
        Self::new("extensions")
    }
}