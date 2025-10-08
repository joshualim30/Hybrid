// lexer.rs
// Tokenizer for the Hybrid language

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Literals
    Number(f64),
    Boolean(bool),
    StringLiteral(String),
    Identifier(String),
    
    // Operators
    Plus,
    Minus,
    Multiply,
    Divide,
    Assign,
    
    // Delimiters
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Semicolon,
    
    // Keywords
    Var,
    Const,
    Block,
    Return,
    Speak,
    
    // Special
    EOF,
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    current_char: Option<char>,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let chars: Vec<char> = input.chars().collect();
        let current_char = chars.get(0).copied();
        
        Lexer {
            input: chars,
            position: 0,
            current_char,
        }
    }
    
    fn advance(&mut self) {
        self.position += 1;
        self.current_char = self.input.get(self.position).copied();
    }
    
    fn peek(&self) -> Option<char> {
        self.input.get(self.position + 1).copied()
    }
    
    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current_char {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }
    
    fn skip_comment(&mut self) {
        // Skip single-line comments starting with //
        if self.current_char == Some('/') && self.peek() == Some('/') {
            while let Some(ch) = self.current_char {
                if ch == '\n' {
                    break;
                }
                self.advance();
            }
        }
    }
    
    fn read_number(&mut self) -> f64 {
        let mut number_str = String::new();
        
        while let Some(ch) = self.current_char {
            if ch.is_ascii_digit() || ch == '.' {
                number_str.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        
        number_str.parse().unwrap_or(0.0)
    }
    
    fn read_identifier(&mut self) -> String {
        let mut identifier = String::new();
        
        while let Some(ch) = self.current_char {
            if ch.is_alphanumeric() || ch == '_' {
                identifier.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        
        identifier
    }
    
    fn read_string(&mut self) -> String {
        let mut string_value = String::new();
        self.advance(); // Skip opening quote
        
        while let Some(ch) = self.current_char {
            if ch == '"' {
                self.advance(); // Skip closing quote
                break;
            } else if ch == '\\' {
                self.advance();
                if let Some(escaped) = self.current_char {
                    match escaped {
                        'n' => string_value.push('\n'),
                        't' => string_value.push('\t'),
                        'r' => string_value.push('\r'),
                        '\\' => string_value.push('\\'),
                        '"' => string_value.push('"'),
                        _ => {
                            string_value.push('\\');
                            string_value.push(escaped);
                        }
                    }
                    self.advance();
                }
            } else {
                string_value.push(ch);
                self.advance();
            }
        }
        
        string_value
    }
    
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        
        while let Some(ch) = self.current_char {
            match ch {
                ' ' | '\t' | '\n' | '\r' => self.skip_whitespace(),
                '/' if self.peek() == Some('/') => self.skip_comment(),
                '+' => {
                    tokens.push(Token::Plus);
                    self.advance();
                }
                '-' => {
                    tokens.push(Token::Minus);
                    self.advance();
                }
                '*' => {
                    tokens.push(Token::Multiply);
                    self.advance();
                }
                '/' => {
                    tokens.push(Token::Divide);
                    self.advance();
                }
                '=' => {
                    tokens.push(Token::Assign);
                    self.advance();
                }
                '(' => {
                    tokens.push(Token::LeftParen);
                    self.advance();
                }
                ')' => {
                    tokens.push(Token::RightParen);
                    self.advance();
                }
                '{' => {
                    tokens.push(Token::LeftBrace);
                    self.advance();
                }
                '}' => {
                    tokens.push(Token::RightBrace);
                    self.advance();
                }
                ',' => {
                    tokens.push(Token::Comma);
                    self.advance();
                }
                ';' => {
                    tokens.push(Token::Semicolon);
                    self.advance();
                }
                '"' => {
                    let string_value = self.read_string();
                    tokens.push(Token::StringLiteral(string_value));
                }
                _ if ch.is_ascii_digit() => {
                    let number = self.read_number();
                    tokens.push(Token::Number(number));
                }
                _ if ch.is_alphabetic() || ch == '_' => {
                    let identifier = self.read_identifier();
                    let token = match identifier.as_str() {
                        "var" => Token::Var,
                        "const" => Token::Const,
                        "block" => Token::Block,
                        "return" => Token::Return,
                        "speak" => Token::Speak,
                        "true" => Token::Boolean(true),
                        "false" => Token::Boolean(false),
                        _ => Token::Identifier(identifier),
                    };
                    tokens.push(token);
                }
                _ => {
                    // Skip unknown characters
                    self.advance();
                }
            }
        }
        
        tokens.push(Token::EOF);
        tokens
    }
}
