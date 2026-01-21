// main.rs
// Hybrid Language Interpreter v0.1.0
// A polyglot runtime that seamlessly integrates with Python and Rust

use clap::{Parser as ClapParser, Subcommand};
use std::fs;
use std::io::{self, Write};

use hybrid::lexer::Lexer;
use hybrid::parser::Parser;
use hybrid::evaluator::{Evaluator, StatementResult};

const VERSION: &str = "0.1.0";
const BANNER: &str = r#"
  _   _       _          _     _ 
 | | | |_   _| |__  _ __(_) __| |
 | |_| | | | | '_ \| '__| |/ _` |
 |  _  | |_| | |_) | |  | | (_| |
 |_| |_|\__, |_.__/|_|  |_|\__,_|
        |___/                    
"#;

#[derive(ClapParser)]
#[command(
    name = "hybrid",
    version = VERSION,
    about = "Hybrid Language Interpreter - A polyglot runtime for Python & Rust",
    long_about = "Hybrid is a modern programming language that seamlessly integrates native code\nwith Python and Rust mutable blocks. Write once, run anywhere with the power\nof multiple runtimes."
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Input file to execute (if no subcommand)
    #[arg(value_name = "FILE")]
    input_file: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a Hybrid source file
    Run {
        /// The file to run
        #[arg(value_name = "FILE")]
        file: String,
    },
    /// Start the interactive REPL
    Repl,
    /// Check environment health and show setup instructions
    Doctor {
        /// Attempt to fix common issues
        #[arg(long)]
        fix: bool,
    },
    /// Initialize a new Hybrid project
    Init {
        /// Project name (defaults to current directory name)
        #[arg(value_name = "NAME")]
        name: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();
    
    match cli.command {
        Some(Commands::Run { file }) => {
            run_file(&file);
        }
        Some(Commands::Repl) => {
            run_repl();
        }
        Some(Commands::Doctor { fix }) => {
            run_doctor(fix);
        }
        Some(Commands::Init { name }) => {
            run_init(name);
        }
        None => {
            if let Some(file) = cli.input_file {
                run_file(&file);
            } else {
                // Show welcome message and start REPL
                print_welcome();
                run_repl();
            }
        }
    }
}

fn run_file(filename: &str) {
    match fs::read_to_string(filename) {
        Ok(content) => {
            if let Err(e) = execute_code(&content) {
                eprintln!("Error: {}", e);
            }
        }
        Err(e) => eprintln!("Could not read file '{}': {}", filename, e),
    }
}

fn print_welcome() {
    println!("{}", BANNER);
    println!("Hybrid Language v{}", VERSION);
    println!("A polyglot runtime for Python & Rust\n");
}

fn run_doctor(fix: bool) {
    println!("🔬 Hybrid Doctor: Checking environment...\n");
    
    let mut issues = Vec::new();
    
    // Check Rust
    print!("Checking Rust... ");
    io::stdout().flush().unwrap();
    match std::process::Command::new("rustc").arg("--version").output() {
        Ok(output) if output.status.success() => {
            println!("✅ {}", String::from_utf8_lossy(&output.stdout).trim());
        }
        _ => {
            println!("❌ Not found");
            issues.push(("Rust", "Install Rust: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"));
        }
    }
    
    // Check Python
    print!("Checking Python... ");
    io::stdout().flush().unwrap();
    match std::process::Command::new("python3").arg("--version").output() {
        Ok(output) if output.status.success() => {
            println!("✅ {}", String::from_utf8_lossy(&output.stdout).trim());
        }
        _ => {
            println!("❌ Not found");
            issues.push(("Python", "Install Python: https://www.python.org/downloads/"));
        }
    }
    
    // Check for Hybrid extension
    print!("Checking VS Code extension... ");
    io::stdout().flush().unwrap();
    let ext_path = dirs::home_dir()
        .map(|p| p.join(".vscode/extensions"))
        .filter(|p| p.exists());
    if ext_path.is_some() {
        println!("✅ VS Code detected");
    } else {
        println!("⚠️  VS Code extensions not found (optional)");
    }
    
    println!();
    
    if issues.is_empty() {
        println!("🎉 All checks passed! Your environment is ready.");
        println!("\nQuick start:");
        println!("  hybrid init myproject     Create a new project");
        println!("  hybrid run file.hyb       Run a Hybrid file");
        println!("  hybrid repl               Start interactive mode");
    } else {
        println!("⚠️  {} issue(s) found:\n", issues.len());
        for (name, instruction) in &issues {
            println!("  {} {}", name, if fix { "- Attempting fix..." } else { "" });
            println!("    {}\n", instruction);
        }
        
        if !fix {
            println!("Run 'hybrid doctor --fix' to attempt automatic fixes.");
        }
    }
}

fn run_init(name: Option<String>) {
    let project_name = name.unwrap_or_else(|| {
        std::env::current_dir()
            .ok()
            .and_then(|p| p.file_name().map(|n| n.to_string_lossy().to_string()))
            .unwrap_or_else(|| "hybrid_project".to_string())
    });
    
    println!("🚀 Initializing Hybrid project: {}\n", project_name);
    
    // Create project structure
    let dirs = ["src", "tests"];
    for dir in &dirs {
        if let Err(e) = fs::create_dir_all(dir) {
            eprintln!("  ❌ Failed to create {}: {}", dir, e);
            return;
        }
        println!("  📁 Created {}/", dir);
    }
    
    // Create main.hyb
    let main_content = format!(r#"// {}/src/main.hyb
// Welcome to Hybrid!

// Native Hybrid code
string const greeting = "Hello from Hybrid!";
speak(greeting);

// Python mutable block example
#python
string block py_greet(string name) {{
    return f"Hello from Python, {{name}}!"
}}

// Rust mutable block example  
#rust
int block rs_double(int n) {{
    let val: i64 = n.parse().unwrap();
    val * 2
}}

// Use the mutable blocks
speak(py_greet("World"));
speak(rs_double(21));
"#, project_name);
    
    if let Err(e) = fs::write("src/main.hyb", main_content) {
        eprintln!("  ❌ Failed to create src/main.hyb: {}", e);
        return;
    }
    println!("  📄 Created src/main.hyb");
    
    // Create README
    let readme = format!(r#"# {}

A Hybrid language project with Python and Rust mutable blocks.

## Getting Started

```bash
# Run the project
hybrid run src/main.hyb

# Start the REPL
hybrid repl

# Check environment
hybrid doctor
```

## Learn More

Hybrid is a polyglot runtime that seamlessly integrates native code with Python and Rust.
"#, project_name);
    
    if let Err(e) = fs::write("README.md", readme) {
        eprintln!("  ❌ Failed to create README.md: {}", e);
        return;
    }
    println!("  📄 Created README.md");
    
    println!("\n✅ Project initialized! Run: hybrid run src/main.hyb");
}

fn run_repl() {
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
                    "help" => print_repl_help(),
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

fn print_repl_help() {
    println!("Hybrid REPL Commands:");
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
    println!("  Arrays:        [1, 2, 3]");
    println!("  Maps:          {{ \"key\": \"value\" }}");
}



fn execute_code(code: &str) -> Result<(), String> {
    let mut evaluator = Evaluator::new();
    let lexer = Lexer::new(code);
    let mut parser = Parser::new(lexer);
    
    match parser.parse() {
        Ok(program) => {
            for statement in program.statements {
                if let Err(e) = evaluator.evaluate_statement(&statement) {
                    return Err(format!("Runtime error: {}", e));
                }
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
                match evaluator.evaluate_statement(&statement) {
                    Ok(StatementResult::Value(val)) => println!("{}", val),
                    Ok(_) => {},
                    Err(e) => return Err(format!("Runtime error: {}", e)),
                }
            }
            Ok(())
        }
        Err(e) => Err(format!("Parse error: {}", e)),
    }
}
