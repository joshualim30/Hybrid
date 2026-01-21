// parser.rs
// Recursive descent parser for the Hybrid language

use crate::ast::{Expr, Stmt, Program, BinaryOp, UnaryOp, HybridType, TypedParam};
use crate::lexer::{Token, Lexer};

#[derive(Debug, Clone)]
pub struct ParseError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} at line {}, col {}", self.message, self.line + 1, self.column + 1)
    }
}

pub struct Parser {
    tokens: Vec<(Token, usize, usize)>,
    current: usize,
    source: String,  // Original source for raw block extraction
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let source = lexer.get_source().to_string();
        let tokens = lexer.tokenize();
        Parser {
            tokens,
            current: 0,
            source,
        }
    }
    
    fn current_token(&self) -> &Token {
        self.tokens.get(self.current).map(|(t, _, _)| t).unwrap_or(&Token::EOF)
    }

    fn current_pos(&self) -> (usize, usize) {
        self.tokens.get(self.current).map(|(_, l, c)| (*l, *c)).unwrap_or((0, 0))
    }
    
    fn advance(&mut self) -> &Token {
        if self.current < self.tokens.len() {
            self.current += 1;
        }
        self.current_token()
    }
    
    fn match_token(&mut self, expected: &Token) -> bool {
        if std::mem::discriminant(self.current_token()) == std::mem::discriminant(expected) {
            self.advance();
            true
        } else {
            false
        }
    }
    
    pub fn parse(&mut self) -> Result<Program, ParseError> {
        let mut statements = Vec::new();
        
        while *self.current_token() != Token::EOF {
            match self.parse_statement() {
                Ok(stmt) => statements.push(stmt),
                Err(e) => return Err(e),
            }
        }
        
        Ok(Program { statements })
    }
    
    fn error<T>(&self, message: &str) -> Result<T, ParseError> {
        let (line, column) = self.current_pos();
        Err(ParseError {
            message: message.to_string(),
            line,
            column,
        })
    }

    fn parse_statement(&mut self) -> Result<Stmt, ParseError> {
        // Check for mutable block: #python, #rust, etc.
        if let Token::Mutable(lang) = self.current_token().clone() {
            self.advance(); // consume #lang
            // Next should be type then block
            if self.is_type_token() {
                let var_type = self.parse_type()?;
                if matches!(self.current_token(), Token::Block) {
                    return self.parse_typed_block_declaration(vec![var_type], Some(lang));
                }
                return self.error("Expected 'block' after type in mutable declaration");
            } else if matches!(self.current_token(), Token::LeftParen) {
                let return_types = self.parse_return_type_tuple()?;
                if matches!(self.current_token(), Token::Block) {
                    return self.parse_typed_block_declaration(return_types, Some(lang));
                }
                return self.error("Expected 'block' after return types in mutable declaration");
            }
            return self.error("Expected type or '(' after #lang");
        }
        
        // Check for type-first syntax: `int var x = 10` or `string block name() {}`
        if self.is_type_token() {
            let var_type = self.parse_type()?;
            
            match self.current_token() {
                Token::Var => self.parse_typed_variable_declaration(var_type, false),
                Token::Const => self.parse_typed_variable_declaration(var_type, true),
                Token::Block => self.parse_typed_block_declaration(vec![var_type], None),
                _ => self.error("Expected 'var', 'const', or 'block' after type"),
            }
        } else if matches!(self.current_token(), Token::LeftParen) {
            // Multi-return block: `(int, string) block name() {}`
            let return_types = self.parse_return_type_tuple()?;
            if !matches!(self.current_token(), Token::Block) {
                return self.error("Expected 'block' after return type tuple");
            }
            self.parse_typed_block_declaration(return_types, None)
        } else {
            match self.current_token() {
                Token::Return => self.parse_return_statement(),
                _ => {
                    let expr = self.parse_expression()?;
                    if self.match_token(&Token::Semicolon) {
                        // Optional semicolon
                    }
                    Ok(Stmt::Expression(expr))
                }
            }
        }
    }
    
    fn is_type_token(&self) -> bool {
        matches!(
            self.current_token(),
            Token::TypeInt | Token::TypeFloat | Token::TypeString | 
            Token::TypeBool | Token::TypeVoid | Token::TypeNull |
            Token::TypeArray | Token::TypeMap
        )
    }
    
    fn parse_type(&mut self) -> Result<HybridType, ParseError> {
        match self.current_token().clone() {
            Token::TypeInt => { self.advance(); Ok(HybridType::Int) }
            Token::TypeFloat => { self.advance(); Ok(HybridType::Float) }
            Token::TypeString => { self.advance(); Ok(HybridType::String) }
            Token::TypeBool => { self.advance(); Ok(HybridType::Bool) }
            Token::TypeVoid => { self.advance(); Ok(HybridType::Void) }
            Token::TypeNull => { self.advance(); Ok(HybridType::Null) }
            Token::TypeArray => {
                self.advance(); // consume 'array'
                if !self.match_token(&Token::LeftBracket) {
                    return self.error("Expected '[' after 'array'");
                }
                let inner = self.parse_type()?;
                if !self.match_token(&Token::RightBracket) {
                    return self.error("Expected ']' after array element type");
                }
                Ok(HybridType::Array(Box::new(inner)))
            }
            Token::TypeMap => {
                self.advance(); // consume 'map'
                if !self.match_token(&Token::LeftBrace) {
                    return self.error("Expected '{' after 'map'");
                }
                let key_type = self.parse_type()?;
                if !self.match_token(&Token::Comma) {
                    return self.error("Expected ',' between map key and value types");
                }
                let value_type = self.parse_type()?;
                if !self.match_token(&Token::RightBrace) {
                    return self.error("Expected '}' after map value type");
                }
                Ok(HybridType::Map(Box::new(key_type), Box::new(value_type)))
            }
            _ => self.error(&format!("Expected type, found {:?}", self.current_token())),
        }
    }
    
    fn parse_return_type_tuple(&mut self) -> Result<Vec<HybridType>, ParseError> {
        self.advance(); // consume '('
        let mut types = Vec::new();
        
        while !matches!(self.current_token(), Token::RightParen) {
            types.push(self.parse_type()?);
            if matches!(self.current_token(), Token::Comma) {
                self.advance();
            } else if !matches!(self.current_token(), Token::RightParen) {
                return self.error("Expected ',' or ')' in return type tuple");
            }
        }
        self.advance(); // consume ')'
        Ok(types)
    }
    
    fn parse_typed_variable_declaration(&mut self, var_type: HybridType, is_const: bool) -> Result<Stmt, ParseError> {
        self.advance(); // consume 'var' or 'const'
        
        if let Token::Identifier(name) = self.current_token().clone() {
            self.advance();
            
            if !self.match_token(&Token::Assign) {
                return self.error("Expected '=' after variable name");
            }
            
            let value = self.parse_expression()?;
            
            if self.match_token(&Token::Semicolon) {
                // Optional semicolon
            }
            
            Ok(Stmt::VariableDeclaration {
                is_const,
                name,
                var_type,
                value,
            })
        } else {
            self.error("Expected identifier after 'var' or 'const'")
        }
    }
    
    fn parse_typed_block_declaration(&mut self, return_types: Vec<HybridType>, foreign_lang: Option<String>) -> Result<Stmt, ParseError> {
        self.advance(); // consume 'block'
        
        if let Token::Identifier(name) = self.current_token().clone() {
            self.advance();
            
            if !self.match_token(&Token::LeftParen) {
                return self.error("Expected '(' after block name");
            }
            
            // Parse typed parameters: (int a, string b)
            let mut parameters: Vec<TypedParam> = Vec::new();
            
            while !matches!(self.current_token(), Token::RightParen) {
                // Expect type first
                let param_type = self.parse_type()?;
                
                // Then identifier
                if let Token::Identifier(param_name) = self.current_token().clone() {
                    parameters.push(TypedParam {
                        name: param_name,
                        param_type,
                    });
                    self.advance();
                    
                    if matches!(self.current_token(), Token::Comma) {
                        self.advance();
                    } else if !matches!(self.current_token(), Token::RightParen) {
                        return self.error("Expected ',' or ')' in parameter list");
                    }
                } else {
                    return self.error("Expected parameter name after type");
                }
            }
            
            if !self.match_token(&Token::RightParen) {
                return self.error("Expected ')' after parameters");
            }
            
            if !self.match_token(&Token::LeftBrace) {
                return self.error("Expected '{' to start block body");
            }
            
            // For foreign blocks, capture raw body
            if foreign_lang.is_some() {
                let raw = self.capture_raw_block_body(&name)?;
                return Ok(Stmt::BlockDeclaration {
                    name,
                    parameters,
                    return_types,
                    body: Vec::new(),
                    is_foreign: true,
                    foreign_lang,
                    raw_body: Some(raw),
                });
            }
            
            // Parse Hybrid body
            let mut body = Vec::new();
            
            while *self.current_token() != Token::RightBrace && *self.current_token() != Token::EOF {
                body.push(self.parse_statement()?);
            }
            
            if !self.match_token(&Token::RightBrace) {
                return self.error("Expected '}' to end block body");
            }
            
            Ok(Stmt::BlockDeclaration {
                name,
                parameters,
                return_types,
                body,
                is_foreign: false,
                foreign_lang: None,
                raw_body: None,
            })
        } else {
            self.error("Expected identifier after 'block'")
        }
    }
    
    /// Capture raw text until matching closing brace (for foreign blocks)
    /// Uses the source string to extract actual code content
    fn capture_raw_block_body(&mut self, block_name: &str) -> Result<String, ParseError> {
        // Find the block definition in source and extract the body
        // Pattern: block_name(...) { ...body... }
        let source = &self.source;
        
        // Find the block name in source
        if let Some(name_pos) = source.find(block_name) {
            // Find the opening brace after the block name
            let after_name = &source[name_pos..];
            if let Some(brace_offset) = after_name.find('{') {
                let body_start = name_pos + brace_offset + 1;
                let remaining = &source[body_start..];
                
                // Find matching closing brace
                let mut brace_depth = 1;
                let mut body_end = 0;
                
                for (i, ch) in remaining.chars().enumerate() {
                    match ch {
                        '{' => brace_depth += 1,
                        '}' => {
                            brace_depth -= 1;
                            if brace_depth == 0 {
                                body_end = i;
                                break;
                            }
                        }
                        _ => {}
                    }
                }
                
                if brace_depth == 0 {
                    let raw_body = remaining[..body_end].trim().to_string();
                    
                    // Advance parser past all tokens until we hit the matching }
                    let mut depth = 1;
                    while depth > 0 {
                        match self.current_token() {
                            Token::LeftBrace => depth += 1,
                            Token::RightBrace => depth -= 1,
                            Token::EOF => return self.error("Unexpected EOF in foreign block"),
                            _ => {}
                        }
                        self.advance();
                    }
                    
                    return Ok(raw_body);
                }
            }
        }
        
        self.error("Could not extract foreign block body")
    }
    
    fn parse_return_statement(&mut self) -> Result<Stmt, ParseError> {
        self.advance(); // consume 'return'
        
        let value = if *self.current_token() == Token::Semicolon || *self.current_token() == Token::EOF {
            None
        } else {
            Some(self.parse_expression()?)
        };
        
        if self.match_token(&Token::Semicolon) {
            // Optional semicolon
        }
        
        Ok(Stmt::Return(value))
    }
    
    fn parse_expression(&mut self) -> Result<Expr, ParseError> {
        self.parse_assignment()
    }

    fn parse_assignment(&mut self) -> Result<Expr, ParseError> {
        let expr = self.parse_equality()?;
        
        if self.match_token(&Token::Assign) {
            let value = self.parse_assignment()?;
            
            if let Expr::Identifier(name) = expr {
                return Ok(Expr::Assign {
                    name,
                    value: Box::new(value),
                });
            }
            
            return self.error("Invalid assignment target");
        }
        
        Ok(expr)
    }
    
    fn parse_equality(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_comparison()?;
        
        while matches!(self.current_token(), Token::Equal | Token::NotEqual) {
            let operator = match self.current_token() {
                Token::Equal => BinaryOp::Equal,
                Token::NotEqual => BinaryOp::NotEqual,
                _ => unreachable!(),
            };
            self.advance();
            
            let right = self.parse_comparison()?;
            left = Expr::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }
        
        Ok(left)
    }
    
    fn parse_comparison(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_additive()?;
        
        while matches!(self.current_token(), Token::LessThan | Token::GreaterThan | Token::LessThanOrEqual | Token::GreaterThanOrEqual) {
            let operator = match self.current_token() {
                Token::LessThan => BinaryOp::LessThan,
                Token::GreaterThan => BinaryOp::GreaterThan,
                Token::LessThanOrEqual => BinaryOp::LessThanOrEqual,
                Token::GreaterThanOrEqual => BinaryOp::GreaterThanOrEqual,
                _ => unreachable!(),
            };
            self.advance();
            
            let right = self.parse_additive()?;
            left = Expr::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }
        
        Ok(left)
    }

    fn parse_additive(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_multiplicative()?;
        
        while matches!(self.current_token(), Token::Plus | Token::Minus) {
            let operator = match self.current_token() {
                Token::Plus => BinaryOp::Add,
                Token::Minus => BinaryOp::Subtract,
                _ => unreachable!(),
            };
            self.advance();
            
            let right = self.parse_multiplicative()?;
            left = Expr::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }
        
        Ok(left)
    }
    
    fn parse_multiplicative(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_unary()?;
        
        while matches!(self.current_token(), Token::Multiply | Token::Divide) {
            let operator = match self.current_token() {
                Token::Multiply => BinaryOp::Multiply,
                Token::Divide => BinaryOp::Divide,
                _ => unreachable!(),
            };
            self.advance();
            
            let right = self.parse_unary()?;
            left = Expr::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }
        
        Ok(left)
    }
    
    fn parse_unary(&mut self) -> Result<Expr, ParseError> {
        match self.current_token() {
            Token::Minus => {
                self.advance();
                let operand = self.parse_unary()?;
                Ok(Expr::Unary {
                    operator: UnaryOp::Negate,
                    operand: Box::new(operand),
                })
            }
            Token::Not => {
                self.advance();
                let operand = self.parse_unary()?;
                Ok(Expr::Unary {
                    operator: UnaryOp::Not,
                    operand: Box::new(operand),
                })
            }
            _ => self.parse_primary(),
        }
    }
    
    // Handles postfix expressions (indexing)
    fn parse_primary(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_base_expr()?;
        
        while *self.current_token() == Token::LeftBracket {
            self.advance(); // consume '['
            let index = self.parse_expression()?;
            if !self.match_token(&Token::RightBracket) {
                return self.error("Expected ']' after index");
            }
            expr = Expr::Index {
                target: Box::new(expr),
                index: Box::new(index),
            };
        }
        
        Ok(expr)
    }
    
    // Handles atomic expressions
    fn parse_base_expr(&mut self) -> Result<Expr, ParseError> {
        match self.current_token().clone() {
            Token::If => self.parse_if_expression(),
            Token::While => self.parse_while_expression(),
            Token::LeftBracket => self.parse_array_literal(),
            Token::LeftBrace => self.parse_map_literal(),
            Token::Number(n) => {
                self.advance();
                Ok(Expr::Number(n))
            }
            Token::Boolean(b) => {
                self.advance();
                Ok(Expr::Boolean(b))
            }
            Token::StringLiteral(s) => {
                self.advance();
                Ok(Expr::StringLiteral(s))
            }
            Token::Identifier(name) => {
                self.advance();
                
                // Check for function call
                if *self.current_token() == Token::LeftParen {
                    self.advance(); // consume '('
                    
                    let mut arguments = Vec::new();
                    
                    while *self.current_token() != Token::RightParen {
                        arguments.push(self.parse_expression()?);
                        
                        if *self.current_token() == Token::Comma {
                            self.advance();
                        } else if *self.current_token() != Token::RightParen {
                            return self.error("Expected ',' or ')' in argument list");
                        }
                    }
                    
                    if !self.match_token(&Token::RightParen) {
                        return self.error("Expected ')' after arguments");
                    }
                    
                    Ok(Expr::FunctionCall { name, arguments })
                } else {
                    Ok(Expr::Identifier(name))
                }
            }
            Token::LeftParen => {
                self.advance(); // consume '('
                let expr = self.parse_expression()?;
                if !self.match_token(&Token::RightParen) {
                    return self.error("Expected ')' after expression");
                }
                Ok(expr)
            }
            Token::Speak => {
                self.advance(); // consume 'speak'
                
                if !self.match_token(&Token::LeftParen) {
                    return self.error("Expected '(' after 'speak'");
                }
                
                let mut arguments = Vec::new();
                
                while *self.current_token() != Token::RightParen {
                    arguments.push(self.parse_expression()?);
                    
                    if *self.current_token() == Token::Comma {
                        self.advance();
                    } else if *self.current_token() != Token::RightParen {
                        return self.error("Expected ',' or ')' in speak arguments");
                    }
                }
                
                if !self.match_token(&Token::RightParen) {
                    return self.error("Expected ')' after speak arguments");
                }
                
                Ok(Expr::FunctionCall {
                    name: "speak".to_string(),
                    arguments,
                })
            }
            _ => self.error(&format!("Unexpected token: {:?}", self.current_token())),
        }
    }
    
    fn parse_array_literal(&mut self) -> Result<Expr, ParseError> {
        self.advance(); // consume '['
        
        let mut elements = Vec::new();
        
        while *self.current_token() != Token::RightBracket && *self.current_token() != Token::EOF {
            elements.push(self.parse_expression()?);
            
            if *self.current_token() == Token::Comma {
                self.advance();
            } else if *self.current_token() != Token::RightBracket {
                return self.error("Expected ',' or ']' in array literal");
            }
        }
        
        if !self.match_token(&Token::RightBracket) {
            return self.error("Expected ']' after array elements");
        }
        
        Ok(Expr::Array(elements))
    }
    
    fn parse_map_literal(&mut self) -> Result<Expr, ParseError> {
        self.advance(); // consume '{'
        
        let mut pairs = Vec::new();
        
        while *self.current_token() != Token::RightBrace && *self.current_token() != Token::EOF {
            let key = self.parse_expression()?;
            
            if !self.match_token(&Token::Colon) {
                return self.error("Expected ':' after map key");
            }
            
            let value = self.parse_expression()?;
            pairs.push((key, value));
            
            if *self.current_token() == Token::Comma {
                self.advance();
            } else if *self.current_token() != Token::RightBrace {
                return self.error("Expected ',' or '}' in map literal");
            }
        }
        
        if !self.match_token(&Token::RightBrace) {
            return self.error("Expected '}' after map pairs");
        }
        
        Ok(Expr::Map(pairs))
    }
    
    fn parse_if_expression(&mut self) -> Result<Expr, ParseError> {
        self.advance(); // consume 'if'
        
        if !self.match_token(&Token::LeftParen) {
            return self.error("Expected '(' after 'if'");
        }
        
        let condition = self.parse_expression()?;
        
        if !self.match_token(&Token::RightParen) {
            return self.error("Expected ')' after if condition");
        }
        
        let then_branch = self.parse_block_stmt()?;
        
        let else_branch = if self.match_token(&Token::Else) {
            Some(Box::new(self.parse_block_stmt()?))
        } else {
            None
        };
        
        Ok(Expr::If {
            condition: Box::new(condition),
            then_branch: Box::new(then_branch),
            else_branch,
        })
    }
    
    fn parse_while_expression(&mut self) -> Result<Expr, ParseError> {
        self.advance(); // consume 'while'
        
        if !self.match_token(&Token::LeftParen) {
            return self.error("Expected '(' after 'while'");
        }
        
        let condition = self.parse_expression()?;
        
        if !self.match_token(&Token::RightParen) {
            return self.error("Expected ')' after while condition");
        }
        
        let body = self.parse_block_stmt()?;
        
        Ok(Expr::While {
            condition: Box::new(condition),
            body: Box::new(body),
        })
    }
    
    fn parse_block_stmt(&mut self) -> Result<Stmt, ParseError> {
        if !self.match_token(&Token::LeftBrace) {
            return self.error("Expected '{' for block body");
        }
        
        let mut body = Vec::new();
        
        while *self.current_token() != Token::RightBrace && *self.current_token() != Token::EOF {
            body.push(self.parse_statement()?);
        }
        
        if !self.match_token(&Token::RightBrace) {
            return self.error("Expected '}' after block body");
        }
        
        Ok(Stmt::Block(body))
    }
}
