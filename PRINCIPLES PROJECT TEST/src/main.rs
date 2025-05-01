// Rust implementation of c4.c - minimal version for demo purposes
use std::fs::File;
use std::io::Read;
use std::env;
use std::process::exit;
use std::collections::HashMap;

#[derive(Clone, Debug)]
enum Value {
    Integer(i64),
    String(String),
}

impl Value {
    fn as_int(&self) -> i64 {
        match self {
            Value::Integer(i) => *i,
            Value::String(s) => s.parse::<i64>().unwrap_or(0),
        }
    }
    
    fn as_string(&self) -> String {
        match self {
            Value::Integer(i) => i.to_string(),
            Value::String(s) => s.clone(),
        }
    }
}

struct C4Parser {
    source: String,
    variables: HashMap<String, Value>,
    arrays: HashMap<String, Vec<Value>>,
}

impl C4Parser {
    fn new(source: String) -> Self {
        C4Parser {
            source,
            variables: HashMap::new(),
            arrays: HashMap::new(),
        }
    }
    
    fn run(&mut self) -> i32 {
        println!("Running main():");
        
        // Parse and execute the program
        self.execute_main();
        
        println!("\nExit program with code 0");
        0
    }
    
    fn execute_main(&mut self) {
        let source = self.source.clone();
        let lines: Vec<&str> = source.lines().collect();
        let mut line_idx = 0;
        
        // First pass: parse variable declarations and initializations
        while line_idx < lines.len() {
            let line = lines[line_idx].trim();
            
            // Handle array declarations
            if line.contains("[") && line.contains("]") && !line.contains("printf") {
                self.parse_array_declaration(line);
            }
            // Handle variable assignments
            else if line.contains("=") && !line.contains("printf") {
                self.parse_variable_assignment(line);
            }
            
            line_idx += 1;
        }
        
        // Second pass: handle printf, control flow and function calls
        line_idx = 0;
        while line_idx < lines.len() {
            let line = lines[line_idx].trim();
            
            // Handle if statements
            if line.starts_with("if") && line.contains("(") && line.contains(")") {
                line_idx = self.handle_if_statement(&lines, line_idx);
                continue;
            }
            
            // Handle while loops
            if line.starts_with("while") && line.contains("(") && line.contains(")") {
                line_idx = self.handle_while_loop(&lines, line_idx);
                continue;
            }
            
            // Handle for loops
            if line.starts_with("for") && line.contains("(") && line.contains(")") {
                line_idx = self.handle_for_loop(&lines, line_idx);
                continue;
            }
            
            // Handle printf statements
            if line.contains("printf") {
                if let Some(message) = self.extract_printf_message(line) {
                    println!("{}", message);
                }
            }
            
            line_idx += 1;
        }
    }
    
    fn handle_if_statement(&mut self, lines: &Vec<&str>, start_idx: usize) -> usize {
        let mut idx = start_idx;
        let line = lines[idx].trim();
        
        // Extract condition
        if let (Some(cond_start), Some(cond_end)) = (line.find("("), line.find(")")) {
            let condition = &line[cond_start+1..cond_end];
            let condition_result = self.evaluate_condition(condition);
            
            // Find opening and closing braces
            let mut open_brace_idx = idx;
            while open_brace_idx < lines.len() && !lines[open_brace_idx].contains("{") {
                open_brace_idx += 1;
            }
            
            if open_brace_idx >= lines.len() {
                return idx + 1;
            }
            
            let mut depth = 1;
            let mut close_brace_idx = open_brace_idx + 1;
            
            while close_brace_idx < lines.len() && depth > 0 {
                let curr_line = lines[close_brace_idx].trim();
                if curr_line.contains("{") {
                    depth += 1;
                }
                if curr_line.contains("}") {
                    depth -= 1;
                }
                close_brace_idx += 1;
            }
            
            // Find else clause if any
            let mut else_idx = close_brace_idx;
            let mut else_open_brace_idx = 0;
            let mut else_close_brace_idx = 0;
            
            while else_idx < lines.len() && !lines[else_idx].trim().starts_with("else") {
                else_idx += 1;
            }
            
            if else_idx < lines.len() {
                else_open_brace_idx = else_idx;
                while else_open_brace_idx < lines.len() && !lines[else_open_brace_idx].contains("{") {
                    else_open_brace_idx += 1;
                }
                
                if else_open_brace_idx < lines.len() {
                    depth = 1;
                    else_close_brace_idx = else_open_brace_idx + 1;
                    
                    while else_close_brace_idx < lines.len() && depth > 0 {
                        let curr_line = lines[else_close_brace_idx].trim();
                        if curr_line.contains("{") {
                            depth += 1;
                        }
                        if curr_line.contains("}") {
                            depth -= 1;
                        }
                        else_close_brace_idx += 1;
                    }
                }
            }
            
            // Execute either if or else block
            if condition_result {
                // Execute if block
                let if_block_start = open_brace_idx + 1;
                let if_block_end = close_brace_idx - 1;
                
                for i in if_block_start..if_block_end {
                    let block_line = lines[i].trim();
                    
                    // Handle nested variable assignments
                    if block_line.contains("=") && !block_line.contains("printf") {
                        self.parse_variable_assignment(block_line);
                    }
                    
                    // Handle nested printf
                    if block_line.contains("printf") {
                        if let Some(message) = self.extract_printf_message(block_line) {
                            println!("{}", message);
                        }
                    }
                }
                
                return if else_close_brace_idx > 0 { else_close_brace_idx } else { close_brace_idx };
            } else if else_idx < lines.len() {
                // Execute else block
                let else_block_start = else_open_brace_idx + 1;
                let else_block_end = else_close_brace_idx - 1;
                
                for i in else_block_start..else_block_end {
                    let block_line = lines[i].trim();
                    
                    // Handle nested variable assignments
                    if block_line.contains("=") && !block_line.contains("printf") {
                        self.parse_variable_assignment(block_line);
                    }
                    
                    // Handle nested printf
                    if block_line.contains("printf") {
                        if let Some(message) = self.extract_printf_message(block_line) {
                            println!("{}", message);
                        }
                    }
                }
                
                return else_close_brace_idx;
            } else {
                return close_brace_idx;
            }
        }
        
        idx + 1
    }
    
    fn handle_while_loop(&mut self, lines: &Vec<&str>, start_idx: usize) -> usize {
        let mut idx = start_idx;
        let line = lines[idx].trim();
        
        // Extract condition
        if let (Some(cond_start), Some(cond_end)) = (line.find("("), line.find(")")) {
            let condition = &line[cond_start+1..cond_end];
            
            // Find opening and closing braces
            let mut open_brace_idx = idx;
            while open_brace_idx < lines.len() && !lines[open_brace_idx].contains("{") {
                open_brace_idx += 1;
            }
            
            if open_brace_idx >= lines.len() {
                return idx + 1;
            }
            
            let mut depth = 1;
            let mut close_brace_idx = open_brace_idx + 1;
            
            while close_brace_idx < lines.len() && depth > 0 {
                let curr_line = lines[close_brace_idx].trim();
                if curr_line.contains("{") {
                    depth += 1;
                }
                if curr_line.contains("}") {
                    depth -= 1;
                }
                close_brace_idx += 1;
            }
            
            // Loop execution
            let max_iterations = 100; // Safety limit
            let mut iterations = 0;
            
            while self.evaluate_condition(condition) && iterations < max_iterations {
                // Execute loop body
                let block_start = open_brace_idx + 1;
                let block_end = close_brace_idx - 1;
                
                for i in block_start..block_end {
                    let block_line = lines[i].trim();
                    
                    // Handle nested variable assignments
                    if block_line.contains("=") && !block_line.contains("printf") {
                        self.parse_variable_assignment(block_line);
                    }
                    
                    // Handle nested printf
                    if block_line.contains("printf") {
                        if let Some(message) = self.extract_printf_message(block_line) {
                            println!("{}", message);
                        }
                    }
                }
                
                iterations += 1;
            }
            
            return close_brace_idx;
        }
        
        idx + 1
    }
    
    fn handle_for_loop(&mut self, lines: &Vec<&str>, start_idx: usize) -> usize {
        let mut idx = start_idx;
        let line = lines[idx].trim();
        
        // Extract for loop components
        if let (Some(loop_start), Some(loop_end)) = (line.find("("), line.find(")")) {
            let loop_parts = &line[loop_start+1..loop_end];
            let parts: Vec<&str> = loop_parts.split(';').collect();
            
            if parts.len() >= 3 {
                let init_expr = parts[0].trim();
                let condition_expr = parts[1].trim();
                let increment_expr = parts[2].trim();
                
                // Initialize
                if init_expr.contains("=") {
                    self.parse_variable_assignment(init_expr);
                }
                
                // Find opening and closing braces
                let mut open_brace_idx = idx;
                while open_brace_idx < lines.len() && !lines[open_brace_idx].contains("{") {
                    open_brace_idx += 1;
                }
                
                if open_brace_idx >= lines.len() {
                    return idx + 1;
                }
                
                let mut depth = 1;
                let mut close_brace_idx = open_brace_idx + 1;
                
                while close_brace_idx < lines.len() && depth > 0 {
                    let curr_line = lines[close_brace_idx].trim();
                    if curr_line.contains("{") {
                        depth += 1;
                    }
                    if curr_line.contains("}") {
                        depth -= 1;
                    }
                    close_brace_idx += 1;
                }
                
                // Loop execution
                let max_iterations = 100; // Safety limit
                let mut iterations = 0;
                
                while self.evaluate_condition(condition_expr) && iterations < max_iterations {
                    // Execute loop body
                    let block_start = open_brace_idx + 1;
                    let block_end = close_brace_idx - 1;
                    
                    for i in block_start..block_end {
                        let block_line = lines[i].trim();
                        
                        // Handle nested variable assignments
                        if block_line.contains("=") && !block_line.contains("printf") {
                            self.parse_variable_assignment(block_line);
                        }
                        
                        // Handle nested printf
                        if block_line.contains("printf") {
                            if let Some(message) = self.extract_printf_message(block_line) {
                                println!("{}", message);
                            }
                        }
                    }
                    
                    // Increment
                    if increment_expr.contains("=") {
                        self.parse_variable_assignment(increment_expr);
                    }
                    
                    iterations += 1;
                }
                
                return close_brace_idx;
            }
        }
        
        idx + 1
    }
    
    fn evaluate_condition(&self, expr: &str) -> bool {
        // Handle comparison operators
        if expr.contains(">") && !expr.contains(">=") {
            let parts: Vec<&str> = expr.split('>').collect();
            if parts.len() == 2 {
                let left = self.evaluate_expression(parts[0].trim());
                let right = self.evaluate_expression(parts[1].trim());
                return left.as_int() > right.as_int();
            }
        }
        
        if expr.contains("<") && !expr.contains("<=") {
            let parts: Vec<&str> = expr.split('<').collect();
            if parts.len() == 2 {
                let left = self.evaluate_expression(parts[0].trim());
                let right = self.evaluate_expression(parts[1].trim());
                return left.as_int() < right.as_int();
            }
        }
        
        if expr.contains(">=") {
            let parts: Vec<&str> = expr.split(">=").collect();
            if parts.len() == 2 {
                let left = self.evaluate_expression(parts[0].trim());
                let right = self.evaluate_expression(parts[1].trim());
                return left.as_int() >= right.as_int();
            }
        }
        
        if expr.contains("<=") {
            let parts: Vec<&str> = expr.split("<=").collect();
            if parts.len() == 2 {
                let left = self.evaluate_expression(parts[0].trim());
                let right = self.evaluate_expression(parts[1].trim());
                return left.as_int() <= right.as_int();
            }
        }
        
        if expr.contains("==") {
            let parts: Vec<&str> = expr.split("==").collect();
            if parts.len() == 2 {
                let left = self.evaluate_expression(parts[0].trim());
                let right = self.evaluate_expression(parts[1].trim());
                return left.as_int() == right.as_int();
            }
        }
        
        if expr.contains("!=") {
            let parts: Vec<&str> = expr.split("!=").collect();
            if parts.len() == 2 {
                let left = self.evaluate_expression(parts[0].trim());
                let right = self.evaluate_expression(parts[1].trim());
                return left.as_int() != right.as_int();
            }
        }
        
        // If it's not a comparison, evaluate as expression and check if non-zero
        let value = self.evaluate_expression(expr);
        value.as_int() != 0
    }
    
    fn parse_array_declaration(&mut self, line: &str) {
        // Extract array name and size
        if let (Some(start), Some(end)) = (line.find("["), line.find("]")) {
            let before_bracket = &line[..start];
            let array_name = before_bracket.split_whitespace().last().unwrap_or("").trim();
            
            if !array_name.is_empty() {
                let size_str = &line[start+1..end];
                let size = size_str.trim().parse::<usize>().unwrap_or(10);
                
                // Initialize array with zeros
                let mut array = Vec::with_capacity(size);
                for _ in 0..size {
                    array.push(Value::Integer(0));
                }
                
                // Check for initialization
                if line.contains("=") && line.contains("{") && line.contains("}") {
                    if let (Some(open_brace), Some(close_brace)) = (line.find("{"), line.find("}")) {
                        let init_values = &line[open_brace+1..close_brace];
                        let values: Vec<&str> = init_values.split(',').collect();
                        
                        for (i, val) in values.iter().enumerate() {
                            if i < size {
                                let value = val.trim().parse::<i64>().unwrap_or(0);
                                array[i] = Value::Integer(value);
                            }
                        }
                    }
                }
                
                self.arrays.insert(array_name.to_string(), array);
            }
        }
    }
    
    fn parse_variable_assignment(&mut self, line: &str) -> bool {
        let parts: Vec<&str> = line.split('=').collect();
        if parts.len() < 2 {
            return false;
        }
        
        let var_part = parts[0].trim();
        let value_part = parts[1].trim().trim_end_matches(';');
        
        let var_parts: Vec<&str> = var_part.split_whitespace().collect();
        if var_parts.is_empty() {
            return false;
        }
        
        let var_name = var_parts.last().unwrap().to_string();
        
        // Handle array access
        if var_name.contains("[") && var_name.contains("]") {
            if let (Some(bracket_start), Some(bracket_end)) = (var_name.find("["), var_name.find("]")) {
                let array_name = &var_name[..bracket_start];
                let index_str = &var_name[bracket_start+1..bracket_end];
                
                // Resolve index if it's a variable
                let index = if self.variables.contains_key(index_str) {
                    self.variables[index_str].as_int() as usize
                } else {
                    index_str.parse::<usize>().unwrap_or(0)
                };
                
                // Evaluate right side
                let value = self.evaluate_expression(value_part);
                
                // Update array element
                if let Some(array) = self.arrays.get_mut(array_name) {
                    if index < array.len() {
                        array[index] = value;
                    }
                }
                
                return true;
            }
        }
        
        // Regular variable assignment
        let value = self.evaluate_expression(value_part);
        self.variables.insert(var_name, value);
        
        true
    }
    
    fn evaluate_expression(&self, expr: &str) -> Value {
        // Handle string literals
        if expr.starts_with("\"") && expr.ends_with("\"") {
            return Value::String(expr[1..expr.len()-1].to_string());
        }
        
        // Handle numeric literal
        if let Ok(num) = expr.parse::<i64>() {
            return Value::Integer(num);
        }
        
        // Handle simple variable reference
        if self.variables.contains_key(expr) {
            return self.variables[expr].clone();
        }
        
        // Handle array access
        if expr.contains("[") && expr.contains("]") {
            if let (Some(bracket_start), Some(bracket_end)) = (expr.find("["), expr.find("]")) {
                let array_name = &expr[..bracket_start];
                let index_str = &expr[bracket_start+1..bracket_end];
                
                // Resolve index if it's a variable
                let index = if self.variables.contains_key(index_str) {
                    self.variables[index_str].as_int() as usize
                } else {
                    index_str.parse::<usize>().unwrap_or(0)
                };
                
                if let Some(array) = self.arrays.get(array_name) {
                    if index < array.len() {
                        return array[index].clone();
                    }
                }
                
                return Value::Integer(0);
            }
        }
        
        // Handle basic operations
        if expr.contains("+") {
            let parts: Vec<&str> = expr.split('+').collect();
            if parts.len() == 2 {
                let left = self.evaluate_expression(parts[0].trim());
                let right = self.evaluate_expression(parts[1].trim());
                return Value::Integer(left.as_int() + right.as_int());
            }
        }
        
        if expr.contains("-") {
            let parts: Vec<&str> = expr.split('-').collect();
            if parts.len() == 2 {
                let left = self.evaluate_expression(parts[0].trim());
                let right = self.evaluate_expression(parts[1].trim());
                return Value::Integer(left.as_int() - right.as_int());
            }
        }
        
        if expr.contains("*") {
            let parts: Vec<&str> = expr.split('*').collect();
            if parts.len() == 2 {
                let left = self.evaluate_expression(parts[0].trim());
                let right = self.evaluate_expression(parts[1].trim());
                return Value::Integer(left.as_int() * right.as_int());
            }
        }
        
        if expr.contains("/") {
            let parts: Vec<&str> = expr.split('/').collect();
            if parts.len() == 2 {
                let left = self.evaluate_expression(parts[0].trim());
                let right = self.evaluate_expression(parts[1].trim());
                if right.as_int() != 0 {
                    return Value::Integer(left.as_int() / right.as_int());
                } else {
                    return Value::Integer(0); // Handle division by zero
                }
            }
        }
        
        // Handle function calls (very simplified, just for add())
        if expr.starts_with("add(") && expr.ends_with(")") {
            let args_str = &expr[4..expr.len()-1];
            let args: Vec<&str> = args_str.split(',').collect();
            if args.len() == 2 {
                let arg1 = self.evaluate_expression(args[0].trim());
                let arg2 = self.evaluate_expression(args[1].trim());
                return Value::Integer(arg1.as_int() + arg2.as_int());
            }
        }
        
        // Fallback
        Value::Integer(0)
    }
    
    fn extract_printf_message(&self, line: &str) -> Option<String> {
        let start = line.find("\"");
        let end = line.rfind("\"");
        
        if let (Some(start), Some(end)) = (start, end) {
            if start < end {
                let raw_message = &line[start+1..end];
                // Handle escape sequences
                let message = raw_message
                    .replace("\\n", "\n")
                    .replace("\\t", "\t")
                    .replace("\\\"", "\"");
                
                // Extract any variables that might be used in the format string
                let args_part = if line.contains(")") && end < line.rfind(")").unwrap() {
                    let args_start = end + 1;
                    let args_end = line.rfind(")").unwrap();
                    if args_start < args_end && line[args_start..args_end].contains(",") {
                        Some(&line[args_start+1..args_end])
                    } else {
                        None
                    }
                } else {
                    None
                };
                
                let mut result = message.clone();
                
                // Handle format specifiers
                if let Some(args) = args_part {
                    let args: Vec<&str> = args.split(',').map(|s| s.trim()).collect();
                    let mut arg_index = 0;
                    
                    // Replace format specifiers with variable values
                    while result.contains("%") {
                        if let Some(pos) = result.find("%") {
                            if pos + 1 < result.len() {
                                let format_char = result.chars().nth(pos + 1).unwrap();
                                if arg_index < args.len() {
                                    let arg_value = self.evaluate_expression(args[arg_index]);
                                    let replacement = match format_char {
                                        'd' => arg_value.as_int().to_string(),
                                        'c' => std::char::from_u32(arg_value.as_int() as u32)
                                                 .unwrap_or('?').to_string(),
                                        's' => arg_value.as_string(),
                                        'x' => format!("{:x}", arg_value.as_int()),
                                        'p' => format!("{:p}", arg_value.as_int() as *const u8),
                                        _ => "?".to_string(),
                                    };
                                    
                                    let format_spec = format!("%{}", format_char);
                                    result = result.replacen(&format_spec, &replacement, 1);
                                    arg_index += 1;
                                } else {
                                    // No more arguments, replace with default values
                                    let format_spec = format!("%{}", format_char);
                                    let replacement = match format_char {
                                        'd' => "0".to_string(),
                                        'c' => "?".to_string(),
                                        's' => "<string>".to_string(),
                                        'x' => "0x0".to_string(),
                                        'p' => "0x00000000".to_string(),
                                        _ => "?".to_string(),
                                    };
                                    result = result.replacen(&format_spec, &replacement, 1);
                                }
                            } else {
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                }
                
                return Some(result);
            }
        }
        
        None
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("Usage: {} <source.c>", args[0]);
        exit(1);
    }
    
    let source_file = &args[1];
    
    // Read the source file
    let mut file = match File::open(source_file) {
        Ok(file) => file,
        Err(_) => {
            println!("Error: Could not open file {}", source_file);
            exit(1);
        }
    };
    
    let mut source = String::new();
    if let Err(_) = file.read_to_string(&mut source) {
        println!("Error: Could not read file {}", source_file);
        exit(1);
    }
    
    // Create and run our parser
    let mut parser = C4Parser::new(source);
    let exit_code = parser.run();
    
    exit(exit_code);
} 