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
    
    // Comparison
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
    Not,
    
    // Delimiters
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,  // [
    RightBracket, // ]
    Comma,
    Colon,        // :
    Semicolon,
    
    // Keywords
    Var,
    Const,
    Block,
    Return,
    If,
    Else,
    While,
    Speak,
    
    // Type keywords
    TypeInt,
    TypeFloat,
    TypeString,
    TypeBool,
    TypeVoid,
    TypeNull,
    TypeArray,
    TypeMap,
    
    // Mutable blocks (foreign code)
    Mutable(String),  // #python, #rust, etc.
    
    // Special
    EOF,
}

pub struct Lexer {
    input: Vec<char>,
    source: String,  // Original source for raw extraction
    position: usize,
    line: usize,
    column: usize,
    current_char: Option<char>,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let chars: Vec<char> = input.chars().collect();
        let current_char = chars.get(0).copied();
        
        Lexer {
            input: chars,
            source: input.to_string(),
            position: 0,
            line: 0,
            column: 0,
            current_char,
        }
    }
    
    /// Get the original source string
    pub fn get_source(&self) -> &str {
        &self.source
    }
    
    fn advance(&mut self) {
        if let Some(ch) = self.current_char {
            if ch == '\n' {
                self.line += 1;
                self.column = 0;
            } else {
                self.column += 1;
            }
        }
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
    
    pub fn tokenize(&mut self) -> Vec<(Token, usize, usize)> {
        let mut tokens = Vec::new();
        
        while let Some(ch) = self.current_char {
            let start_line = self.line;
            let start_col = self.column;
            
            match ch {
                ' ' | '\t' | '\n' | '\r' => self.skip_whitespace(),
                '/' if self.peek() == Some('/') => self.skip_comment(),
                '+' => {
                    tokens.push((Token::Plus, start_line, start_col));
                    self.advance();
                }
                '-' => {
                    tokens.push((Token::Minus, start_line, start_col));
                    self.advance();
                }
                '*' => {
                    tokens.push((Token::Multiply, start_line, start_col));
                    self.advance();
                }
                '/' => {
                    tokens.push((Token::Divide, start_line, start_col));
                    self.advance();
                }
                '=' => {
                    if self.peek() == Some('=') {
                        self.advance();
                        tokens.push((Token::Equal, start_line, start_col));
                    } else {
                        tokens.push((Token::Assign, start_line, start_col));
                    }
                    self.advance();
                }
                '!' => {
                    if self.peek() == Some('=') {
                        self.advance();
                        tokens.push((Token::NotEqual, start_line, start_col));
                    } else {
                        tokens.push((Token::Not, start_line, start_col));
                    }
                    self.advance();
                }
                '<' => {
                    if self.peek() == Some('=') {
                        self.advance();
                        tokens.push((Token::LessThanOrEqual, start_line, start_col));
                    } else {
                        tokens.push((Token::LessThan, start_line, start_col));
                    }
                    self.advance();
                }
                '>' => {
                    if self.peek() == Some('=') {
                        self.advance();
                        tokens.push((Token::GreaterThanOrEqual, start_line, start_col));
                    } else {
                        tokens.push((Token::GreaterThan, start_line, start_col));
                    }
                    self.advance();
                }
                '(' => {
                    tokens.push((Token::LeftParen, start_line, start_col));
                    self.advance();
                }
                ')' => {
                    tokens.push((Token::RightParen, start_line, start_col));
                    self.advance();
                }
                '{' => {
                    tokens.push((Token::LeftBrace, start_line, start_col));
                    self.advance();
                }
                '}' => {
                    tokens.push((Token::RightBrace, start_line, start_col));
                    self.advance();
                }
                '[' => {
                    tokens.push((Token::LeftBracket, start_line, start_col));
                    self.advance();
                }
                ']' => {
                    tokens.push((Token::RightBracket, start_line, start_col));
                    self.advance();
                }
                ',' => {
                    tokens.push((Token::Comma, start_line, start_col));
                    self.advance();
                }
                ':' => {
                    tokens.push((Token::Colon, start_line, start_col));
                    self.advance();
                }
                ';' => {
                    tokens.push((Token::Semicolon, start_line, start_col));
                    self.advance();
                }
                '#' => {
                    // Mutable block tag: #python, #rust, etc.
                    self.advance(); // consume '#'
                    if let Some(c) = self.current_char {
                        if c.is_alphabetic() {
                            let lang = self.read_identifier();
                            tokens.push((Token::Mutable(lang), start_line, start_col));
                        }
                    }
                }
                '"' => {
                    let string_value = self.read_string();
                    tokens.push((Token::StringLiteral(string_value), start_line, start_col));
                }
                _ if ch.is_ascii_digit() => {
                    let number = self.read_number();
                    tokens.push((Token::Number(number), start_line, start_col));
                }
                _ if ch.is_alphabetic() || ch == '_' => {
                    let identifier = self.read_identifier();
                    let token = match identifier.as_str() {
                        "var" => Token::Var,
                        "const" => Token::Const,
                        "block" => Token::Block,
                        "return" => Token::Return,
                        "if" => Token::If,
                        "else" => Token::Else,
                        "while" => Token::While,
                        "speak" => Token::Speak,
                        "true" => Token::Boolean(true),
                        "false" => Token::Boolean(false),
                        // Type keywords
                        "int" => Token::TypeInt,
                        "float" => Token::TypeFloat,
                        "string" => Token::TypeString,
                        "bool" => Token::TypeBool,
                        "void" => Token::TypeVoid,
                        "null" => Token::TypeNull,
                        "array" => Token::TypeArray,
                        "map" => Token::TypeMap,
                        _ => Token::Identifier(identifier),
                    };
                    tokens.push((token, start_line, start_col));
                }
                _ => {
                    // Skip unknown characters
                    self.advance();
                }
            }
        }
        
        tokens.push((Token::EOF, self.line, self.column));
        tokens
    }
}
