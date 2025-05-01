#[cfg(test)]
mod tests {
    use std::process::Command;
    use std::fs::File;
    use std::io::Write;
    
    #[test]
    fn test_hello_world() {
        // Create hello.c
        let mut file = File::create("hello_test.c").unwrap();
        file.write_all(b"int main() {\n    printf(\"Hello, World!\\n\");\n    return 0;\n}\n").unwrap();
        
        // Build the project
        Command::new("cargo")
            .args(&["build"])
            .status()
            .expect("Failed to build project");
        
        // Run the hello world program
        let output = Command::new("./target/debug/c4_rust")
            .arg("hello_test.c")
            .output()
            .expect("Failed to execute c4_rust");
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        
        // Check if the output contains "Hello, World!"
        assert!(stdout.contains("Hello, World!"));
    }
} 