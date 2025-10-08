// main.rs
// Hybrid Language Interpreter v0.1.0

use std::env;
use std::fs;
use std::io::{self, Write};

mod lexer;
mod ast;
mod parser;
mod evaluator;

use lexer::Lexer;
use parser::Parser;
use evaluator::Evaluator;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() > 1 {
        // File execution mode
        let filename = &args[1];
        match fs::read_to_string(filename) {
            Ok(content) => {
                if let Err(e) = execute_code(&content) {
                    eprintln!("Error: {}", e);
                }
            }
            Err(e) => eprintln!("Could not read file '{}': {}", filename, e),
        }
    } else {
        // REPL mode
        println!("Hybrid Language Interpreter v0.1.0");
        println!("Type 'help' for commands or 'exit' to quit");
        
        let mut evaluator = Evaluator::new();
        
        loop {
            print!("hybrid> ");
            io::stdout().flush().unwrap();
            
            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(_) => {
                    let input = input.trim();
                    
                    if input.is_empty() {
                        continue;
                    }
                    
                    match input {
                        "exit" | "quit" => break,
                        "help" => print_help(),
                        _ => {
                            if let Err(e) = execute_line(&mut evaluator, input) {
                                eprintln!("Error: {}", e);
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error reading input: {}", e);
                    break;
                }
            }
        }
    }
}

fn execute_code(code: &str) -> Result<(), String> {
    let mut evaluator = Evaluator::new();
    let lexer = Lexer::new(code);
    let mut parser = Parser::new(lexer);
    
    match parser.parse() {
        Ok(program) => {
            for statement in program.statements {
                evaluator.evaluate_statement(&statement)?;
            }
            Ok(())
        }
        Err(e) => Err(format!("Parse error: {}", e)),
    }
}

fn execute_line(evaluator: &mut Evaluator, code: &str) -> Result<(), String> {
    let lexer = Lexer::new(code);
    let mut parser = Parser::new(lexer);
    
    match parser.parse() {
        Ok(program) => {
            for statement in program.statements {
                let result = evaluator.evaluate_statement(&statement)?;
                if let Some(value) = result {
                    println!("{}", value);
                }
            }
            Ok(())
        }
        Err(e) => Err(format!("Parse error: {}", e)),
    }
}

fn print_help() {
    println!("Hybrid Language Commands:");
    println!("  help           - Show this help message");
    println!("  exit/quit      - Exit the interpreter");
    println!();
    println!("Basic Syntax:");
    println!("  Numbers:       42, 3.14");
    println!("  Variables:     var x = 10;");
    println!("  Constants:     const y = 20;");
    println!("  Arithmetic:    2 + 3 * 4");
    println!("  Functions:     block add(a, b) {{ return a + b; }}");
    println!("  Print:         speak(\"Hello World\");");
    println!();
    println!("Examples:");
    println!("  var x = 5;");
    println!("  const y = x * 2 + 3;");
    println!("  block double(n) {{ return n * 2; }}");
    println!("  speak(double(21));");
}
