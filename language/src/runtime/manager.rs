// runtime/manager.rs
// Runtime Manager - coordinates foreign runtime execution

use std::collections::HashMap;
use std::process::Command;

/// Represents a value that can be passed to/from foreign runtimes
#[derive(Debug, Clone)]
pub enum RuntimeValue {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Null,
}

/// Error from foreign runtime execution
#[derive(Debug)]
pub struct RuntimeError {
    pub language: String,
    pub message: String,
}

impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.language, self.message)
    }
}

/// Manages foreign runtime processes
pub struct RuntimeManager {
    available_runtimes: HashMap<String, bool>,
}

impl RuntimeManager {
    /// Create a new RuntimeManager, checking which runtimes are available
    pub fn new() -> Self {
        let mut available = HashMap::new();
        
        // Check Python
        available.insert("python".to_string(), Self::check_python());
        
        // Check Rust
        available.insert("rust".to_string(), Self::check_rust());
        
        RuntimeManager {
            available_runtimes: available,
        }
    }
    
    fn check_python() -> bool {
        Command::new("python3")
            .arg("--version")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }
    
    fn check_rust() -> bool {
        Command::new("rustc")
            .arg("--version")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }
    
    /// Check if a runtime is available
    pub fn is_available(&self, lang: &str) -> bool {
        self.available_runtimes.get(lang).copied().unwrap_or(false)
    }
    
    /// Execute a foreign block
    pub fn execute(
        &self,
        lang: &str,
        _block_name: &str,
        code: &str,
        param_names: &[String],
        args: Vec<RuntimeValue>,
    ) -> Result<Vec<RuntimeValue>, RuntimeError> {
        if !self.is_available(lang) {
            return Err(RuntimeError {
                language: lang.to_string(),
                message: format!("Runtime '{}' is not available", lang),
            });
        }
        
        match lang {
            "python" => self.execute_python(code, param_names, args),
            "rust" => self.execute_rust(code, param_names, args),
            _ => Err(RuntimeError {
                language: lang.to_string(),
                message: format!("Unsupported runtime: {}", lang),
            }),
        }
    }
    
    fn execute_python(&self, code: &str, param_names: &[String], args: Vec<RuntimeValue>) -> Result<Vec<RuntimeValue>, RuntimeError> {
        // Build parameter assignment lines
        let param_assignments: Vec<String> = param_names
            .iter()
            .enumerate()
            .map(|(i, name)| format!("{} = args[{}]", name, i))
            .collect();
        let param_setup = param_assignments.join("\n");
        
        // Build the Python wrapper script with named parameters
        let wrapper = format!(r#"
import json
import sys

# Arguments passed from Hybrid
args = json.loads(sys.argv[1])

# Inject parameters as variables
{param_setup}

# User's function body (execute and capture result)
def __hybrid_fn():
{code}

result = __hybrid_fn()
print(json.dumps(result if result is not None else None))
"#, param_setup = param_setup, code = Self::indent_code(code, "    "));
        
        let args_json = serde_json::to_string(&Self::values_to_json(&args))
            .map_err(|e| RuntimeError {
                language: "python".to_string(),
                message: format!("Failed to serialize args: {}", e),
            })?;
        
        let output = Command::new("python3")
            .arg("-c")
            .arg(&wrapper)
            .arg(&args_json)
            .output()
            .map_err(|e| RuntimeError {
                language: "python".to_string(),
                message: format!("Failed to execute Python: {}", e),
            })?;
        
        if !output.status.success() {
            return Err(RuntimeError {
                language: "python".to_string(),
                message: String::from_utf8_lossy(&output.stderr).to_string(),
            });
        }
        
        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
        Self::parse_json_result(&stdout, "python")
    }
    
    fn execute_rust(&self, code: &str, param_names: &[String], args: Vec<RuntimeValue>) -> Result<Vec<RuntimeValue>, RuntimeError> {
        use std::fs;
        use std::env::temp_dir;
        
        // Build parameter assignment lines
        let param_assignments: Vec<String> = param_names
            .iter()
            .enumerate()
            .map(|(i, name)| {
                format!("    let {} = &args[{}];", name, i)
            })
            .collect();
        let param_setup = param_assignments.join("\n");
        
        // Build Rust wrapper
        let wrapper = format!(r#"
use std::env;

fn main() {{
    let args: Vec<String> = env::args().skip(1).collect();
{param_setup}
    
    // User's function body
    let result = (|| {{
{code}
    }})();
    
    // Output result as JSON
    println!("{{}}", serde_json_lite(&result));
}}

fn serde_json_lite<T: std::fmt::Debug>(v: &T) -> String {{
    format!("{{:?}}", v)
}}
"#, param_setup = param_setup, code = Self::indent_code(code, "        "));
        
        // Write to temp file
        let temp_path = temp_dir().join("hybrid_rust_block.rs");
        let binary_path = temp_dir().join("hybrid_rust_block");
        
        fs::write(&temp_path, &wrapper).map_err(|e| RuntimeError {
            language: "rust".to_string(),
            message: format!("Failed to write temp file: {}", e),
        })?;
        
        // Compile
        let compile_output = Command::new("rustc")
            .arg(&temp_path)
            .arg("-o")
            .arg(&binary_path)
            .output()
            .map_err(|e| RuntimeError {
                language: "rust".to_string(),
                message: format!("Failed to run rustc: {}", e),
            })?;
        
        if !compile_output.status.success() {
            return Err(RuntimeError {
                language: "rust".to_string(),
                message: String::from_utf8_lossy(&compile_output.stderr).to_string(),
            });
        }
        
        // Execute with args
        let arg_strings: Vec<String> = args.iter().map(|v| match v {
            RuntimeValue::Int(n) => n.to_string(),
            RuntimeValue::Float(n) => n.to_string(),
            RuntimeValue::String(s) => s.clone(),
            RuntimeValue::Bool(b) => b.to_string(),
            RuntimeValue::Null => "null".to_string(),
        }).collect();
        
        let run_output = Command::new(&binary_path)
            .args(&arg_strings)
            .output()
            .map_err(|e| RuntimeError {
                language: "rust".to_string(),
                message: format!("Failed to run binary: {}", e),
            })?;
        
        // Clean up temp files
        let _ = fs::remove_file(&temp_path);
        let _ = fs::remove_file(&binary_path);
        
        if !run_output.status.success() {
            return Err(RuntimeError {
                language: "rust".to_string(),
                message: String::from_utf8_lossy(&run_output.stderr).to_string(),
            });
        }
        
        let stdout = String::from_utf8_lossy(&run_output.stdout).trim().to_string();
        
        // Parse simple output (Rust debug format)
        Ok(vec![Self::parse_rust_output(&stdout)])
    }
    
    fn parse_rust_output(output: &str) -> RuntimeValue {
        // Simple parsing of Rust debug output
        if output == "true" {
            RuntimeValue::Bool(true)
        } else if output == "false" {
            RuntimeValue::Bool(false)
        } else if let Ok(n) = output.parse::<i64>() {
            RuntimeValue::Int(n)
        } else if let Ok(n) = output.parse::<f64>() {
            RuntimeValue::Float(n)
        } else if output.starts_with('"') && output.ends_with('"') {
            RuntimeValue::String(output[1..output.len()-1].to_string())
        } else {
            RuntimeValue::String(output.to_string())
        }
    }
    
    fn indent_code(code: &str, indent: &str) -> String {
        code.lines()
            .map(|line| format!("{}{}", indent, line))
            .collect::<Vec<_>>()
            .join("\n")
    }
    
    fn values_to_json(values: &[RuntimeValue]) -> Vec<serde_json::Value> {
        values.iter().map(|v| match v {
            RuntimeValue::Int(n) => serde_json::json!(n),
            RuntimeValue::Float(n) => serde_json::json!(n),
            RuntimeValue::String(s) => serde_json::json!(s),
            RuntimeValue::Bool(b) => serde_json::json!(b),
            RuntimeValue::Null => serde_json::Value::Null,
        }).collect()
    }
    
    fn parse_json_result(json_str: &str, lang: &str) -> Result<Vec<RuntimeValue>, RuntimeError> {
        let value: serde_json::Value = serde_json::from_str(json_str)
            .map_err(|e| RuntimeError {
                language: lang.to_string(),
                message: format!("Failed to parse result: {}", e),
            })?;
        
        Ok(vec![Self::json_to_value(value)])
    }
    
    fn json_to_value(v: serde_json::Value) -> RuntimeValue {
        match v {
            serde_json::Value::Null => RuntimeValue::Null,
            serde_json::Value::Bool(b) => RuntimeValue::Bool(b),
            serde_json::Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    RuntimeValue::Int(i)
                } else if let Some(f) = n.as_f64() {
                    RuntimeValue::Float(f)
                } else {
                    RuntimeValue::Null
                }
            }
            serde_json::Value::String(s) => RuntimeValue::String(s),
            _ => RuntimeValue::Null, // Arrays and objects not yet supported
        }
    }
}
