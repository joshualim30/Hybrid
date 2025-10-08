// parser.rs
// Recursive descent parser for the Hybrid language

use crate::ast::{Expr, Stmt, Program, BinaryOp, UnaryOp};
use crate::lexer::{Token, Lexer};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let tokens = lexer.tokenize();
        Parser {
            tokens,
            current: 0,
        }
    }
    
    fn current_token(&self) -> &Token {
        self.tokens.get(self.current).unwrap_or(&Token::EOF)
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
    
    pub fn parse(&mut self) -> Result<Program, String> {
        let mut statements = Vec::new();
        
        while *self.current_token() != Token::EOF {
            match self.parse_statement() {
                Ok(stmt) => statements.push(stmt),
                Err(e) => return Err(e),
            }
        }
        
        Ok(Program { statements })
    }
    
    fn parse_statement(&mut self) -> Result<Stmt, String> {
        match self.current_token() {
            Token::Var => self.parse_variable_declaration(false),
            Token::Const => self.parse_variable_declaration(true),
            Token::Block => self.parse_block_declaration(),
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
    
    fn parse_variable_declaration(&mut self, is_const: bool) -> Result<Stmt, String> {
        self.advance(); // consume 'var' or 'const'
        
        if let Token::Identifier(name) = self.current_token().clone() {
            self.advance();
            
            if !self.match_token(&Token::Assign) {
                return Err("Expected '=' after variable name".to_string());
            }
            
            let value = self.parse_expression()?;
            
            if self.match_token(&Token::Semicolon) {
                // Optional semicolon
            }
            
            Ok(Stmt::VariableDeclaration {
                is_const,
                name,
                value,
            })
        } else {
            Err("Expected identifier after 'var' or 'const'".to_string())
        }
    }
    
    fn parse_block_declaration(&mut self) -> Result<Stmt, String> {
        self.advance(); // consume 'block'
        
        if let Token::Identifier(name) = self.current_token().clone() {
            self.advance();
            
            if !self.match_token(&Token::LeftParen) {
                return Err("Expected '(' after block name".to_string());
            }
            
            let mut parameters = Vec::new();
            
            while *self.current_token() != Token::RightParen {
                if let Token::Identifier(param) = self.current_token().clone() {
                    parameters.push(param);
                    self.advance();
                    
                    if *self.current_token() == Token::Comma {
                        self.advance();
                    } else if *self.current_token() != Token::RightParen {
                        return Err("Expected ',' or ')' in parameter list".to_string());
                    }
                } else {
                    return Err("Expected parameter name".to_string());
                }
            }
            
            if !self.match_token(&Token::RightParen) {
                return Err("Expected ')' after parameters".to_string());
            }
            
            if !self.match_token(&Token::LeftBrace) {
                return Err("Expected '{' to start block body".to_string());
            }
            
            let mut body = Vec::new();
            
            while *self.current_token() != Token::RightBrace && *self.current_token() != Token::EOF {
                body.push(self.parse_statement()?);
            }
            
            if !self.match_token(&Token::RightBrace) {
                return Err("Expected '}' to end block body".to_string());
            }
            
            Ok(Stmt::BlockDeclaration {
                name,
                parameters,
                body,
            })
        } else {
            Err("Expected identifier after 'block'".to_string())
        }
    }
    
    fn parse_return_statement(&mut self) -> Result<Stmt, String> {
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
    
    fn parse_expression(&mut self) -> Result<Expr, String> {
        self.parse_additive()
    }
    
    fn parse_additive(&mut self) -> Result<Expr, String> {
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
    
    fn parse_multiplicative(&mut self) -> Result<Expr, String> {
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
    
    fn parse_unary(&mut self) -> Result<Expr, String> {
        match self.current_token() {
            Token::Minus => {
                self.advance();
                let operand = self.parse_unary()?;
                Ok(Expr::Unary {
                    operator: UnaryOp::Negate,
                    operand: Box::new(operand),
                })
            }
            _ => self.parse_primary(),
        }
    }
    
    fn parse_primary(&mut self) -> Result<Expr, String> {
        match self.current_token().clone() {
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
                            return Err("Expected ',' or ')' in argument list".to_string());
                        }
                    }
                    
                    if !self.match_token(&Token::RightParen) {
                        return Err("Expected ')' after arguments".to_string());
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
                    return Err("Expected ')' after expression".to_string());
                }
                Ok(expr)
            }
            Token::Speak => {
                self.advance(); // consume 'speak'
                
                if !self.match_token(&Token::LeftParen) {
                    return Err("Expected '(' after 'speak'".to_string());
                }
                
                let mut arguments = Vec::new();
                
                while *self.current_token() != Token::RightParen {
                    arguments.push(self.parse_expression()?);
                    
                    if *self.current_token() == Token::Comma {
                        self.advance();
                    } else if *self.current_token() != Token::RightParen {
                        return Err("Expected ',' or ')' in speak arguments".to_string());
                    }
                }
                
                if !self.match_token(&Token::RightParen) {
                    return Err("Expected ')' after speak arguments".to_string());
                }
                
                Ok(Expr::FunctionCall {
                    name: "speak".to_string(),
                    arguments,
                })
            }
            _ => Err(format!("Unexpected token: {:?}", self.current_token())),
        }
    }
}
