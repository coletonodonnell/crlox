use crate::token::{Token, TokenType, Literal};
use std::collections::HashMap;

// Scanner, takes a Lox instance, a source, a token vector associated with it, a start, current, and line.
#[derive(Default)]
pub struct Scanner {
    pub instance: crate::Lox,
    pub source: String,
    pub source_chars: Vec<char>,
    pub tokens: Vec<Token>,
    pub start: u32,
    pub current: u32,
    pub line: u32,
}

// Simple builder for a scanner.
pub fn scanner_builder(instance: crate::Lox, input: String) -> Scanner {
    let mut scanner: Scanner = Scanner {
        instance: instance,
        source: input,
        source_chars: Vec::new(),
        tokens: Vec::new(),
        start: 0,
        current: 0,
        line: 1,
    };
    
    scanner.source_chars = scanner.source.chars().collect::<Vec<char>>();
    return scanner;
}

// Scaner Implementation
impl Scanner {
    // What is ran 
    pub fn scan_tokens(&mut self) -> Vec<Token> {
        // While we aren't at the end of the file, set the start to the current and then scan the next token.
        while !Self::is_end(&self) {
            self.start = self.current;
            Self::scan_token(self);
        }

        // Push a token at the end that represents the end of the file.
        self.tokens.push(Token {token_type: TokenType::Eof, lexeme: String::from(""), literal: None::<Literal>, line: self.line});

        return self.tokens.clone();
    }

    fn scan_token(&mut self) {
        let c: char = Self::advance(self);
    
        // Match the next char with the following
        match c {
            '(' => Self::add_token(self, TokenType::LParen, None::<Literal>),
            ')' => Self::add_token(self, TokenType::RParen, None::<Literal>),
            '{' => Self::add_token(self, TokenType::LBrace, None::<Literal>),
            '}' => Self::add_token(self, TokenType::RBrace, None::<Literal>),
            ',' => Self::add_token(self, TokenType::Comma, None::<Literal>),
            '.' => Self::add_token(self, TokenType::Dot, None::<Literal>),
            '-' => Self::add_token(self, TokenType::Minus, None::<Literal>),
            '+' => Self::add_token(self, TokenType::Plus, None::<Literal>),
            ':' => Self::add_token(self, TokenType::Colon, None::<Literal>),
            ';' => Self::add_token(self, TokenType::Semicolon, None::<Literal>),
            '*' => Self::add_token(self, TokenType::Star, None::<Literal>),
            '?' => Self::add_token(self, TokenType::Question, None::<Literal>),
            '!' => {
                if Self::find_next(self, '=') {
                    Self::add_token(self, TokenType::BangEqual, None::<Literal>);

                } else {
                    Self::add_token(self, TokenType::Bang, None::<Literal>);
                }
            },
            '=' => {
                if Self::find_next(self, '=') {
                    Self::add_token(self, TokenType::EqualEqual, None::<Literal>);

                } else {
                    Self::add_token(self, TokenType::Equal, None::<Literal>);
                }
            },
            '<' => {
                if Self::find_next(self, '=') {
                    Self::add_token(self, TokenType::LessEqual, None::<Literal>);

                } else {
                    Self::add_token(self, TokenType::Less, None::<Literal>);
                }
            },
            '>' => {
                if Self::find_next(self, '=') {
                    Self::add_token(self, TokenType::GreaterEqual, None::<Literal>);

                } else {
                    Self::add_token(self, TokenType::Greater, None::<Literal>);
                }
            },
            '/' => {
                // One line comment?
                if Self::find_next(self, '/') {
                    while Self::peak(&self) != '\n' && !Self::is_end(&self) {
                        Self::advance(self);
                    } 

                // Block comment?
                } else if Self::find_next(self, '*') {
                    // 
                    loop {
                        Self::advance(self);

                        if Self::peak(&self) == '\n' {
                            self.line = self.line + 1;
                        }

                        if Self::peak(&self) == '*' {
                            Self::advance(self);
                            if Self::peak(&self) == '/' {
                                Self::advance(self);
                                break;
                            }
                        }

                        if Self::is_end(self) {
                            self.instance.scanner_error(self.line, "Undetermined Block Comment");
                        }
                    }
                    
                // Div?
                } else {
                    Self::add_token(self, TokenType::Slash, None::<Literal>);
                }
            },
            '"' => Self::string(self),
            ' ' => {},
            '\r' => {},
            '\t' => {},
            '\n' => {self.line = self.line + 1},
            _ => {
                // If it is a digit
                if Self::is_digit(c) {
                    // Declare it as a number
                    Self::number(self);
                // If it is a letter
                } else if Self::is_alpha(c) {
                    // Declare it as a letter
                    Self::identifier(self);
                // Grab the instance of Scanner and then declare a error of "Unexpected character."
                } else {
                    self.instance.scanner_error(self.line, "Unexpected character.");
                }
            },
        }
    }

    // Are we at the end of the file/prompt?
    fn is_end(&self) -> bool {
        return self.current >= self.source.len() as u32;
    }

    fn advance(&mut self) -> char {
        self.current = self.current + 1;
        return self.source_chars[(self.current - 1) as usize];
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<Literal>) {
        // Declare the lexeme.
        let lexeme: String = self.source.get(self.start as usize..self.current as usize).unwrap().to_string();

        // Push the token with the new lexeme.
        self.tokens.push(Token {token_type: token_type, lexeme: lexeme, literal: literal, line: self.line});
    }
    
    fn find_next(&mut self, expected: char) -> bool {
        if Self::is_end(&self) {
            return false;
        }

        if self.source_chars[self.current as usize] != expected {
            return false;
        }

        self.current = self.current + 1;
        return true;
    }

    // This finds the next, or peaks, the next character
    fn peak(&self) -> char {
        if Self::is_end(&self) {
            return '\0';
        } else {
            return self.source_chars[self.current as usize];
        }
    }

    fn string(&mut self) {
        // While we are before the closing " and we're not at the end, look for new lines and advance
        while Self::peak(&self) != '"' && !Self::is_end(&self) {
            if Self::peak(&self) == '\n' {
                self.line = self.line + 1;
            }
            Self::advance(self);
        }

        // Is this the end? That isn't good, please error.
        if Self::is_end(&self) {
            self.instance.scanner_error(self.line, "Undetermined String");
            return;
        }

        Self::advance(self);
        let value: String = self.source.get((self.start + 1) as usize..(self.current -1) as usize).unwrap().to_string();
        Self::add_token(self, TokenType::String, Some(Literal::Str(value)));
    }

    fn number(&mut self) {
        // While it is a number, continue
        while Self::is_digit(Self::peak(self)) {
            Self::advance(self);
        }

        // If there is a decimal and a number following, advance and continue.
        if Self::peak(self) == '.' && Self::is_digit(Self::peak_next(self)) {
            Self::advance(self);

            while Self::is_digit(Self::peak(self)) {
                Self::advance(self);
            }
        }

        let num: String = self.source.get(self.start as usize..self.current as usize).unwrap().to_string();
        Self::add_token(self, TokenType::Num, Some(Literal::Num(num.parse().unwrap())));
    }

    fn is_digit(c: char) -> bool {
        return c >= '0' && c <= '9';
    }

    fn peak_next(&self) -> char {
        // If the count is greater than or equal to the length of source length
        if self.current + 1 >= self.source.len() as u32 {
            return '\0'
        }

        return self.source_chars[(self.current + 1) as usize];
    }

    fn identifier(&mut self) {
        // While the chars are alphanumerical
        while Self::is_alpha_numeric(Self::peak(self)) {
            Self::advance(self);
        }

        // Setup a hashmap of keywords
        let mut keywords = HashMap::new();
        keywords.insert("and", TokenType::And);
        keywords.insert("class", TokenType::Class);
        keywords.insert("else", TokenType::Else);
        keywords.insert("false", TokenType::False);
        keywords.insert("for", TokenType::For);
        keywords.insert("fun", TokenType::Fun);
        keywords.insert("if", TokenType::If);
        keywords.insert("nil", TokenType::Nil);
        keywords.insert("or", TokenType::Or);
        keywords.insert("print", TokenType::Print);
        keywords.insert("return", TokenType::Return);
        keywords.insert("super", TokenType::Super);
        keywords.insert("this", TokenType::This);
        keywords.insert("true", TokenType::True);
        keywords.insert("var", TokenType::Var);
        keywords.insert("while", TokenType::While);

        // Locate the string relating to this hashmap
        let text: &str = self.source.get(self.start as usize..self.current as usize).unwrap();
        let token_type: &TokenType;

        // Match the keywords
        match keywords.get(text) {
            // If it is basically any of the above, go ahead and unwrap it and declare token_type as that token.
            Some(a) => {
                token_type = a;
            }
            // If it is none of the above, set it as an ID
            None => {
                token_type = &TokenType::Id;
            }
        }

        // Add the token.
        Self::add_token(self, token_type.clone(), None);
    }

    // Is this Char an Alpha (or _)
    fn is_alpha(c: char) -> bool {
        return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_';
    }

    // Is this Char either an Alpha/_ or a Numeric?
    fn is_alpha_numeric(c: char) -> bool {
        return Self::is_alpha(c) || c.is_numeric();
    }
}