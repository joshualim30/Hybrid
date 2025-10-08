// evaluator.rs
// Evaluator for the Hybrid language

use std::collections::HashMap;
use crate::ast::{Expr, Stmt, BinaryOp, UnaryOp};

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    Boolean(bool),
    String(String),
    Null,
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(n) => {
                if n.fract() == 0.0 {
                    write!(f, "{}", *n as i64)
                } else {
                    write!(f, "{}", n)
                }
            }
            Value::Boolean(b) => write!(f, "{}", b),
            Value::String(s) => write!(f, "{}", s),
            Value::Null => write!(f, "null"),
        }
    }
}

#[derive(Clone)]
pub struct Function {
    pub parameters: Vec<String>,
    pub body: Vec<Stmt>,
}

pub struct Evaluator {
    variables: HashMap<String, Value>,
    functions: HashMap<String, Function>,
}

impl Evaluator {
    pub fn new() -> Self {
        Evaluator {
            variables: HashMap::new(),
            functions: HashMap::new(),
        }
    }
    
    pub fn evaluate_statement(&mut self, stmt: &Stmt) -> Result<Option<Value>, String> {
        match stmt {
            Stmt::Expression(expr) => {
                let value = self.evaluate_expression(expr)?;
                Ok(Some(value))
            }
            Stmt::VariableDeclaration { is_const: _, name, value } => {
                let val = self.evaluate_expression(value)?;
                self.variables.insert(name.clone(), val);
                Ok(None)
            }
            Stmt::BlockDeclaration { name, parameters, body } => {
                let function = Function {
                    parameters: parameters.clone(),
                    body: body.clone(),
                };
                self.functions.insert(name.clone(), function);
                Ok(None)
            }
            Stmt::Return(expr) => {
                if let Some(expr) = expr {
                    let value = self.evaluate_expression(expr)?;
                    Ok(Some(value))
                } else {
                    Ok(Some(Value::Null))
                }
            }
        }
    }
    
    pub fn evaluate_expression(&mut self, expr: &Expr) -> Result<Value, String> {
        match expr {
            Expr::Number(n) => Ok(Value::Number(*n)),
            Expr::Boolean(b) => Ok(Value::Boolean(*b)),
            Expr::StringLiteral(s) => Ok(Value::String(s.clone())),
            Expr::Identifier(name) => {
                self.variables.get(name)
                    .cloned()
                    .ok_or_else(|| format!("Undefined variable: {}", name))
            }
            Expr::Binary { left, operator, right } => {
                let left_val = self.evaluate_expression(left)?;
                let right_val = self.evaluate_expression(right)?;
                self.evaluate_binary_op(&left_val, operator, &right_val)
            }
            Expr::Unary { operator, operand } => {
                let val = self.evaluate_expression(operand)?;
                self.evaluate_unary_op(operator, &val)
            }
            Expr::FunctionCall { name, arguments } => {
                if name == "speak" {
                    // Built-in speak function
                    let mut output = String::new();
                    for (i, arg) in arguments.iter().enumerate() {
                        if i > 0 {
                            output.push(' ');
                        }
                        let val = self.evaluate_expression(arg)?;
                        output.push_str(&val.to_string());
                    }
                    println!("{}", output);
                    Ok(Value::Null)
                } else {
                    // User-defined function
                    self.call_function(name, arguments)
                }
            }
        }
    }
    
    fn evaluate_binary_op(&self, left: &Value, op: &BinaryOp, right: &Value) -> Result<Value, String> {
        match (left, right) {
            (Value::Number(l), Value::Number(r)) => {
                match op {
                    BinaryOp::Add => Ok(Value::Number(l + r)),
                    BinaryOp::Subtract => Ok(Value::Number(l - r)),
                    BinaryOp::Multiply => Ok(Value::Number(l * r)),
                    BinaryOp::Divide => {
                        if *r == 0.0 {
                            Err("Division by zero".to_string())
                        } else {
                            Ok(Value::Number(l / r))
                        }
                    }
                }
            }
            (Value::String(l), Value::String(r)) => {
                match op {
                    BinaryOp::Add => Ok(Value::String(format!("{}{}", l, r))),
                    _ => Err(format!("Cannot apply {:?} to strings", op)),
                }
            }
            (Value::String(l), other) | (other, Value::String(l)) => {
                match op {
                    BinaryOp::Add => Ok(Value::String(format!("{}{}", l, other))),
                    _ => Err(format!("Cannot apply {:?} to string and {:?}", op, other)),
                }
            }
            _ => Err(format!("Cannot apply {:?} to {:?} and {:?}", op, left, right)),
        }
    }
    
    fn evaluate_unary_op(&self, op: &UnaryOp, operand: &Value) -> Result<Value, String> {
        match (op, operand) {
            (UnaryOp::Negate, Value::Number(n)) => Ok(Value::Number(-n)),
            _ => Err(format!("Cannot apply {:?} to {:?}", op, operand)),
        }
    }
    
    fn call_function(&mut self, name: &str, arguments: &[Expr]) -> Result<Value, String> {
        // Check if function exists first
        if !self.functions.contains_key(name) {
            return Err(format!("Undefined function: {}", name));
        }
        
        // Get function info without borrowing the whole function
        let param_count = self.functions[name].parameters.len();
        
        if arguments.len() != param_count {
            return Err(format!(
                "Function {} expects {} arguments, got {}",
                name,
                param_count,
                arguments.len()
            ));
        }
        
        // Evaluate arguments first
        let mut arg_values = Vec::new();
        for arg in arguments {
            let value = self.evaluate_expression(arg)?;
            arg_values.push(value);
        }
        
        // Now clone the function completely
        let function = self.functions[name].clone();
        
        // Save current variable state
        let saved_vars = self.variables.clone();
        
        // Bind arguments to parameters
        for (param, value) in function.parameters.iter().zip(arg_values.iter()) {
            self.variables.insert(param.clone(), value.clone());
        }
        
        // Execute function body
        let mut result = Value::Null;
        for stmt in &function.body {
            if let Some(value) = self.evaluate_statement(stmt)? {
                result = value;
                // Check if this is a return statement
                if matches!(stmt, Stmt::Return(_)) {
                    break;
                }
            }
        }
        
        // Restore variable state
        self.variables = saved_vars;
        
        Ok(result)
    }
}
