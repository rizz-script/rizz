mod simple_runtime;

use rizz_core::parser::parse_program;
use simple_runtime::SimpleRuntime;
use wasm_bindgen::prelude::*;

// Set panic hook for better error messages in the browser
#[wasm_bindgen(start)]
pub fn init() {
    console_error_panic_hook::set_once();
}

/// Execution result returned to JavaScript
#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct ExecutionResult {
    success: bool,
    output: Vec<String>,
    error: Option<String>,
}

#[wasm_bindgen]
impl ExecutionResult {
    #[wasm_bindgen(getter)]
    pub fn success(&self) -> bool {
        self.success
    }

    #[wasm_bindgen(getter)]
    pub fn output(&self) -> Vec<String> {
        self.output.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn error(&self) -> Option<String> {
        self.error.clone()
    }
}

/// RizzScript interpreter for WASM
#[wasm_bindgen]
pub struct RizzInterpreter {
    // Runtime is created fresh for each execution
}

#[wasm_bindgen]
impl RizzInterpreter {
    /// Create a new interpreter instance
    #[wasm_bindgen(constructor)]
    pub fn new() -> RizzInterpreter {
        RizzInterpreter {}
    }

    /// Execute RizzScript code and return the result
    /// Output is written to the browser console and captured in the result
    #[wasm_bindgen]
    pub fn execute(&self, code: &str) -> ExecutionResult {
        // Parse the program
        let ast = match parse_program(code, "playground.rizz") {
            Ok(ast) => ast,
            Err(e) => {
                return ExecutionResult {
                    success: false,
                    output: vec![],
                    error: Some(format!("Parse error: {}", e)),
                };
            }
        };

        // Create runtime
        let mut runtime = SimpleRuntime::new();

        // Execute the program
        match runtime.run(&ast) {
            Ok(_) => ExecutionResult {
                success: true,
                output: runtime.get_output(),
                error: None,
            },
            Err(e) => ExecutionResult {
                success: false,
                output: runtime.get_output(),
                error: Some(format!("Runtime error: {}", e)),
            },
        }
    }
}

impl Default for RizzInterpreter {
    fn default() -> Self {
        Self::new()
    }
}

/// Execute RizzScript code directly (convenience function)
/// This creates a new interpreter and executes the code
#[wasm_bindgen]
pub fn execute_rizz(code: &str) -> ExecutionResult {
    let interpreter = RizzInterpreter::new();
    interpreter.execute(code)
}

/// Parse RizzScript code and check for syntax errors
/// Returns null if parsing succeeds, or an error message
#[wasm_bindgen]
pub fn check_syntax(code: &str) -> Option<String> {
    match parse_program(code, "playground.rizz") {
        Ok(_) => None,
        Err(e) => Some(format!("{}", e)),
    }
}

/// Get the RizzScript version
#[wasm_bindgen]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

/// Log a message to the browser console
#[wasm_bindgen]
pub fn log(s: &str) {
    web_sys::console::log_1(&s.into());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_syntax_check_valid() {
        assert!(check_syntax("Rizz(\"Hello\")").is_none());
    }

    #[test]
    fn test_syntax_check_invalid() {
        // Missing closing paren is a syntax error
        assert!(check_syntax("Rizz(\"Hello\"").is_some());
        // Invalid token
        assert!(check_syntax("@#$%").is_some());
    }

    #[test]
    fn test_version() {
        assert!(!version().is_empty());
        assert_eq!(version(), "0.1.0");
    }

    // Tests below only work on wasm32 target due to wasm-bindgen
    #[test]
    #[cfg(target_arch = "wasm32")]
    fn test_basic_execution() {
        let interp = RizzInterpreter::new();
        let result = interp.execute("Rizz(\"Hello, World!\")");
        assert!(result.success());
        assert_eq!(result.output(), vec!["Hello, World!"]);
    }

    #[test]
    #[cfg(target_arch = "wasm32")]
    fn test_variables() {
        let interp = RizzInterpreter::new();
        let result = interp.execute("Ayo x = 42\nRizz(x)");
        assert!(result.success());
        assert_eq!(result.output(), vec!["42"]);
    }

    #[test]
    #[cfg(target_arch = "wasm32")]
    fn test_string_concatenation() {
        let interp = RizzInterpreter::new();
        let result = interp.execute("Ayo msg = \"Hello\" + \" \" + \"World\"\nRizz(msg)");
        assert!(result.success());
        assert_eq!(result.output(), vec!["Hello World"]);
    }

    #[test]
    #[cfg(target_arch = "wasm32")]
    fn test_arrays() {
        let interp = RizzInterpreter::new();
        let result = interp.execute("Ayo arr = [1, 2, 3]\nRizz(arr)");
        assert!(result.success());
        assert!(!result.output().is_empty());
    }

    #[test]
    #[cfg(not(target_arch = "wasm32"))]
    fn test_error_handling_native() {
        // Test on native that parsing undefined variable succeeds (it's valid syntax)
        assert!(check_syntax("Rizz(undefined_variable)").is_none());
    }
}
