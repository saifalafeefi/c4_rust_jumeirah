use std::path::Path;
use std::fs::File;
use std::io::Write;
use std::process::Command;

#[cfg(test)]
mod lexer_tests {
    use super::*;
    
    // Helper function to create a test file and run it through our C4 interpreter
    fn run_test_file(name: &str, content: &str) -> String {
        // Create a temporary test file
        let test_path = format!("{}.c", name);
        let mut file = File::create(&test_path).unwrap();
        file.write_all(content.as_bytes()).unwrap();
        
        // Build and run the project
        let output = Command::new("cargo")
            .args(&["run", "--", &test_path])
            .output()
            .expect("Failed to execute c4_rust");
        
        // Clean up
        std::fs::remove_file(&test_path).ok();
        
        String::from_utf8_lossy(&output.stdout).to_string()
    }
    
    #[test]
    fn test_numeric_literals() {
        let test_content = r#"
        int main() {
            int a = 10;
            int b = 0x1A;  // Hex
            int c = 075;   // Octal
            printf("a=%d, b=%d, c=%d\n", a, b, c);
            return 0;
        }
        "#;
        
        let output = run_test_file("numeric_literals", test_content);
        
        // Check that the numbers were properly parsed
        assert!(output.contains("a=10, b=26, c=61") || 
                output.contains("a=10") && output.contains("b=26") && output.contains("c=61"));
    }
    
    #[test]
    fn test_string_literals() {
        let test_content = r#"
        int main() {
            printf("Simple string\n");
            printf("String with \"quotes\"\n");
            printf("String with \n escape sequences\n");
            return 0;
        }
        "#;
        
        let output = run_test_file("string_literals", test_content);
        
        // Check that the strings were properly parsed
        assert!(output.contains("Simple string"));
        assert!(output.contains("String with") && output.contains("quotes"));
        assert!(output.contains("String with") && output.contains("escape sequences"));
    }
    
    #[test]
    fn test_operators() {
        let test_content = r#"
        int main() {
            int a = 5;
            int b = 10;
            
            printf("a + b = %d\n", a + b);
            printf("a - b = %d\n", a - b);
            printf("a * b = %d\n", a * b);
            printf("b / a = %d\n", b / a);
            printf("b %% a = %d\n", b % a);
            
            printf("a < b: %d\n", a < b);
            printf("a > b: %d\n", a > b);
            printf("a <= b: %d\n", a <= b);
            printf("a >= b: %d\n", a >= b);
            printf("a == b: %d\n", a == b);
            printf("a != b: %d\n", a != b);
            
            return 0;
        }
        "#;
        
        let output = run_test_file("operators", test_content);
        
        // Check that operators were properly parsed
        assert!(output.contains("a + b = 15"));
        assert!(output.contains("a - b = -5"));
        assert!(output.contains("a * b = 50"));
        assert!(output.contains("b / a = 2"));
        
        // Logical operators might return 1/0 or true/false
        assert!(output.contains("a < b: 1") || output.contains("a < b: true"));
        assert!(output.contains("a > b: 0") || output.contains("a > b: false"));
    }
    
    #[test]
    fn test_identifiers() {
        let test_content = r#"
        int main() {
            int simple = 1;
            int _with_underscore = 2;
            int with123numbers = 3;
            int UPPERCASE = 4;
            int camelCase = 5;
            
            printf("simple=%d, _with_underscore=%d, with123numbers=%d, UPPERCASE=%d, camelCase=%d\n",
                simple, _with_underscore, with123numbers, UPPERCASE, camelCase);
            
            return 0;
        }
        "#;
        
        let output = run_test_file("identifiers", test_content);
        
        // Check that identifiers were properly parsed
        assert!(output.contains("simple=1"));
        assert!(output.contains("_with_underscore=2"));
        assert!(output.contains("with123numbers=3"));
        assert!(output.contains("UPPERCASE=4"));
        assert!(output.contains("camelCase=5"));
    }
} 