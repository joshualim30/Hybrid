// evaluator.rs
// Evaluator for the Hybrid language

use std::collections::HashMap;
use crate::ast::{Expr, Stmt, BinaryOp, UnaryOp};

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    Boolean(bool),
    String(String),
    Array(Vec<Value>),
    Map(HashMap<String, Value>),
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
            Value::String(s) => write!(f, "\"{}\"", s), // Quote strings
            Value::Array(arr) => {
                write!(f, "[")?;
                for (i, v) in arr.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}", v)?;
                }
                write!(f, "]")
            }
            Value::Map(map) => {
                write!(f, "{{")?;
                for (i, (k, v)) in map.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "\"{}\": {}", k, v)?;
                }
                write!(f, "}}")
            }
            Value::Null => write!(f, "null"),
        }
    }
}

#[derive(Clone)]
pub struct Function {
    pub parameters: Vec<String>,
    pub body: Vec<Stmt>,
}

/// A foreign function defined with #lang
#[derive(Clone)]
pub struct ForeignFunction {
    pub parameters: Vec<String>,
    pub raw_code: String,
    pub language: String,
}

#[derive(Clone, Debug)]
pub struct VariableInfo {
    pub value: Value,
    pub is_const: bool,
}

use crate::runtime::manager::{RuntimeManager, RuntimeValue};

pub struct Evaluator {
    variables: HashMap<String, VariableInfo>,
    functions: HashMap<String, Function>,
    foreign_functions: HashMap<String, ForeignFunction>,
    runtime: RuntimeManager,
}
#[derive(Debug, Clone, PartialEq)]
pub enum StatementResult {
    None,
    Value(Value),
}

#[derive(Debug)]
pub enum EvalError {
    Message(String),
    Return(Value),
}

impl From<String> for EvalError {
    fn from(s: String) -> Self {
        EvalError::Message(s)
    }
}

impl From<&str> for EvalError {
    fn from(s: &str) -> Self {
        EvalError::Message(s.to_string())
    }
}

// Display impl for friendlier error printing
impl std::fmt::Display for EvalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EvalError::Message(msg) => write!(f, "{}", msg),
            EvalError::Return(val) => write!(f, "Uncaught return: {}", val),
        }
    }
}

impl Evaluator {
    pub fn new() -> Self {
        Evaluator {
            variables: HashMap::new(),
            functions: HashMap::new(),
            foreign_functions: HashMap::new(),
            runtime: RuntimeManager::new(),
        }
    }
    
    pub fn evaluate_statement(&mut self, stmt: &Stmt) -> Result<StatementResult, EvalError> {
        match stmt {
            Stmt::Expression(expr) => {
                let value = self.evaluate_expression(expr)?;
                Ok(StatementResult::Value(value))
            }
            Stmt::VariableDeclaration { is_const, name, value, .. } => {
                let val = self.evaluate_expression(value)?;
                self.variables.insert(name.clone(), VariableInfo { value: val, is_const: *is_const });
                Ok(StatementResult::None)
            }
            Stmt::BlockDeclaration { name, parameters, body, is_foreign, foreign_lang, raw_body, .. } => {
                let param_names: Vec<String> = parameters.iter().map(|p| p.name.clone()).collect();
                
                if *is_foreign {
                    // Store as foreign function
                    let foreign_fn = ForeignFunction {
                        parameters: param_names,
                        raw_code: raw_body.clone().unwrap_or_default(),
                        language: foreign_lang.clone().unwrap_or_default(),
                    };
                    self.foreign_functions.insert(name.clone(), foreign_fn);
                } else {
                    // Store as native Hybrid function
                    let function = Function {
                        parameters: param_names,
                        body: body.clone(),
                    };
                    self.functions.insert(name.clone(), function);
                }
                Ok(StatementResult::None)
            }
            Stmt::Return(expr) => {
                if let Some(expr) = expr {
                    let value = self.evaluate_expression(expr)?;
                    // Propagate return immediately as an error to unwind
                    Err(EvalError::Return(value))
                } else {
                    Err(EvalError::Return(Value::Null))
                }
            }
            Stmt::Block(stmts) => {
                let mut last_value = StatementResult::None;
                for stmt in stmts {
                    let result = self.evaluate_statement(stmt)?;
                    match result {
                        StatementResult::Value(_) => last_value = result,
                        StatementResult::None => {}
                    }
                }
                Ok(last_value)
            }
        }
    }
    
    pub fn evaluate_expression(&mut self, expr: &Expr) -> Result<Value, EvalError> {
        match expr {
            Expr::Number(n) => Ok(Value::Number(*n)),
            Expr::Boolean(b) => Ok(Value::Boolean(*b)),
            Expr::StringLiteral(s) => Ok(Value::String(s.clone())),
            Expr::Identifier(name) => {
                self.variables.get(name)
                    .map(|info| info.value.clone())
                    .ok_or_else(|| EvalError::from(format!("Undefined variable: {}", name)))
            }
            Expr::Assign { name, value } => {
                if !self.variables.contains_key(name) {
                    return Err(EvalError::from(format!("Undefined variable: {}", name)));
                }
                
                let info = self.variables.get(name).unwrap();
                if info.is_const {
                    return Err(EvalError::from(format!("Cannot reassign constant '{}'", name)));
                }
                
                let val = self.evaluate_expression(value)?;
                self.variables.insert(name.clone(), VariableInfo { value: val.clone(), is_const: false });
                Ok(val)
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
                    self.call_function(name, arguments)
                }
            }
            Expr::If { condition, then_branch, else_branch } => {
                let cond_val = self.evaluate_expression(condition)?;
                if let Value::Boolean(b) = cond_val {
                    if b {
                        match self.evaluate_statement(then_branch)? {
                            StatementResult::Value(v) => Ok(v),
                            StatementResult::None => Ok(Value::Null),
                        }
                    } else if let Some(else_stmt) = else_branch {
                        match self.evaluate_statement(else_stmt)? {
                            StatementResult::Value(v) => Ok(v),
                            StatementResult::None => Ok(Value::Null),
                        }
                    } else {
                        Ok(Value::Null)
                    }
                } else {
                    Err(EvalError::from("If condition must be a boolean"))
                }
            }
            Expr::While { condition, body } => {
                let mut last_val = Value::Null;
                loop {
                    let cond_val = self.evaluate_expression(condition)?;
                    if let Value::Boolean(b) = cond_val {
                        if !b {
                            break;
                        }
                        
                        match self.evaluate_statement(body)? {
                             StatementResult::Value(v) => last_val = v,
                             StatementResult::None => {},
                        }
                    } else {
                        return Err(EvalError::from("While condition must be a boolean"));
                    }
                }
                Ok(last_val)
            }
            Expr::Array(elements) => {
                let mut values = Vec::new();
                for expr in elements {
                    values.push(self.evaluate_expression(expr)?);
                }
                Ok(Value::Array(values))
            }
            Expr::Map(pairs) => {
                let mut map = HashMap::new();
                for (key_expr, val_expr) in pairs {
                    let key = self.evaluate_expression(key_expr)?;
                    let value = self.evaluate_expression(val_expr)?;
                    
                    match key {
                        Value::String(s) => { map.insert(s, value); },
                        _ => return Err(EvalError::from("Map keys must be strings")),
                    }
                }
                Ok(Value::Map(map))
            }
            Expr::Index { target, index } => {
                let target_val = self.evaluate_expression(target)?;
                let index_val = self.evaluate_expression(index)?;
                
                match target_val {
                    Value::Array(arr) => {
                        match index_val {
                            Value::Number(n) => {
                                if n.fract() != 0.0 {
                                    return Err(EvalError::from("Array index must be an integer"));
                                }
                                let idx = n as usize; // Safety check needed?
                                if n < 0.0 || idx >= arr.len() {
                                    return Err(EvalError::from(format!("Index {} out of bounds (len {})", n, arr.len())));
                                }
                                Ok(arr[idx].clone())
                            }
                            _ => Err(EvalError::from("Array index must be a number")),
                        }
                    }
                    Value::Map(map) => {
                        match index_val {
                            Value::String(s) => {
                                Ok(map.get(&s).cloned().unwrap_or(Value::Null))
                            }
                            _ => Err(EvalError::from("Map index must be a string")),
                        }
                    }
                    _ => Err(EvalError::from("Cannot index non-collection type")),
                }
            }
        }
    }
    
    fn evaluate_binary_op(&self, left: &Value, op: &BinaryOp, right: &Value) -> Result<Value, EvalError> {
        match (left, right) {
            (Value::Number(l), Value::Number(r)) => {
                match op {
                    BinaryOp::Add => Ok(Value::Number(l + r)),
                    BinaryOp::Subtract => Ok(Value::Number(l - r)),
                    BinaryOp::Multiply => Ok(Value::Number(l * r)),
                    BinaryOp::Divide => {
                        if *r == 0.0 {
                            Err(EvalError::from("Division by zero"))
                        } else {
                            Ok(Value::Number(l / r))
                        }
                    }
                    BinaryOp::Equal => Ok(Value::Boolean(l == r)),
                    BinaryOp::NotEqual => Ok(Value::Boolean(l != r)),
                    BinaryOp::LessThan => Ok(Value::Boolean(l < r)),
                    BinaryOp::GreaterThan => Ok(Value::Boolean(l > r)),
                    BinaryOp::LessThanOrEqual => Ok(Value::Boolean(l <= r)),
                    BinaryOp::GreaterThanOrEqual => Ok(Value::Boolean(l >= r)),
                }
            }
            (Value::String(l), Value::String(r)) => {
                match op {
                    BinaryOp::Add => Ok(Value::String(format!("{}{}", l, r))),
                    BinaryOp::Equal => Ok(Value::Boolean(l == r)),
                    BinaryOp::NotEqual => Ok(Value::Boolean(l != r)),
                    _ => Err(EvalError::from(format!("Cannot apply {:?} to strings", op))),
                }
            }
            (Value::Boolean(l), Value::Boolean(r)) => {
                match op {
                    BinaryOp::Equal => Ok(Value::Boolean(l == r)),
                    BinaryOp::NotEqual => Ok(Value::Boolean(l != r)),
                    _ => Err(EvalError::from(format!("Cannot apply {:?} to booleans", op))),
                }
            }
            (Value::Null, Value::Null) => {
                match op {
                    BinaryOp::Equal => Ok(Value::Boolean(true)),
                    BinaryOp::NotEqual => Ok(Value::Boolean(false)),
                    _ => Err(EvalError::from(format!("Cannot apply {:?} to null", op))),
                }
            }
            _ => {
                match op {
                    BinaryOp::Equal => Ok(Value::Boolean(false)),
                    BinaryOp::NotEqual => Ok(Value::Boolean(true)),
                    _ => Err(EvalError::from(format!("Cannot apply {:?} to {:?} and {:?}", op, left, right))),
                }
            }
        }
    }
    
    fn evaluate_unary_op(&self, op: &UnaryOp, operand: &Value) -> Result<Value, EvalError> {
        match (op, operand) {
            (UnaryOp::Negate, Value::Number(n)) => Ok(Value::Number(-n)),
            (UnaryOp::Not, Value::Boolean(b)) => Ok(Value::Boolean(!b)),
            _ => Err(EvalError::from(format!("Cannot apply {:?} to {:?}", op, operand))),
        }
    }
    
    fn call_function(&mut self, name: &str, arguments: &[Expr]) -> Result<Value, EvalError> {
        // Check for foreign function first
        if let Some(foreign_fn) = self.foreign_functions.get(name).cloned() {
            let mut arg_values = Vec::new();
            for arg in arguments {
                let value = self.evaluate_expression(arg)?;
                arg_values.push(Self::value_to_runtime(&value));
            }
            
            return match self.runtime.execute(&foreign_fn.language, name, &foreign_fn.raw_code, &foreign_fn.parameters, arg_values) {
                Ok(results) => {
                    if let Some(first) = results.into_iter().next() {
                        Ok(Self::runtime_to_value(first))
                    } else {
                        Ok(Value::Null)
                    }
                }
                Err(e) => Err(EvalError::from(format!("[{}] {}", e.language, e.message))),
            };
        }
        
        if !self.functions.contains_key(name) {
            return Err(EvalError::from(format!("Undefined function: {}", name)));
        }
        
        let param_count = self.functions[name].parameters.len();
        
        if arguments.len() != param_count {
            return Err(EvalError::from(format!(
                "Function {} expects {} arguments, got {}",
                name,
                param_count,
                arguments.len()
            )));
        }
        
        let mut arg_values = Vec::new();
        for arg in arguments {
            let value = self.evaluate_expression(arg)?;
            arg_values.push(value);
        }
        
        let function = self.functions[name].clone();
        let saved_vars = self.variables.clone();
        
        for (param, value) in function.parameters.iter().zip(arg_values.iter()) {
            self.variables.insert(param.clone(), VariableInfo { value: value.clone(), is_const: false });
        }
        
        let mut result = Value::Null;
        for stmt in &function.body {
            match self.evaluate_statement(stmt) {
                Ok(res) => {
                    if let StatementResult::Value(v) = res {
                        result = v;
                    } 
                    // StatementResult::None -> do nothing
                    // StatementResult::Return -> THIS SHOULD BE EvalError::Return?
                    // No, evaluate_statement now implementation triggers Err(EvalError::Return) on Return stmt.
                    // So Ok(StatementResult::Return) is impossible by my Stmt::Return logic.
                    // Wait, see Stmt::Return impl:
                    // Err(EvalError::Return(value))
                    // So evaluate_statement returns Err for Return!
                }
                Err(e) => {
                    match e {
                        EvalError::Return(val) => {
                            // Caught the return!
                            result = val;
                            break;
                        }
                        EvalError::Message(_) => return Err(e), // Propagate errors
                    }
                }
            }
        }
        
        self.variables = saved_vars;
        Ok(result)
    }
    
    /// Convert Hybrid Value to RuntimeValue
    fn value_to_runtime(value: &Value) -> RuntimeValue {
        match value {
            Value::Number(n) => {
                if n.fract() == 0.0 {
                    RuntimeValue::Int(*n as i64)
                } else {
                    RuntimeValue::Float(*n)
                }
            }
            Value::Boolean(b) => RuntimeValue::Bool(*b),
            Value::String(s) => RuntimeValue::String(s.clone()),
            Value::Null => RuntimeValue::Null,
            _ => RuntimeValue::Null, // Arrays/Maps not yet supported
        }
    }
    
    /// Convert RuntimeValue to Hybrid Value
    fn runtime_to_value(rv: RuntimeValue) -> Value {
        match rv {
            RuntimeValue::Int(n) => Value::Number(n as f64),
            RuntimeValue::Float(n) => Value::Number(n),
            RuntimeValue::Bool(b) => Value::Boolean(b),
            RuntimeValue::String(s) => Value::String(s),
            RuntimeValue::Null => Value::Null,
        }
    }
}

