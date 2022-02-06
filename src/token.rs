use std::fmt;

#[derive(Clone, Debug)]
pub enum Literal {
    Str(String),
    Num(f64),
    False,
    True,
    Nill
}

// Each type of Token
#[derive(Clone, Debug, PartialEq)]
pub enum TokenType {
    // Single Character
    LParen,
    RParen,
    LBrace,
    RBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    Colon,
    Question,

    // Compare
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Id,
    String,
    Num,

    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    Eof
}

// Token struct, consisting of a TokenType, a String, a Literal (if applicable) and a corresponding line (for error checking).
#[derive(Clone, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<Literal>,
    pub line: u32,
}

// Display a Literal
impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Str(a) => write!(f, "{:?}", a),
            Self::Num(a) => write!(f, "{:?}", a),
            Self::False => write!(f, "false"),
            Self::True => write!(f, "true"),
            Self::Nill => write!(f, "nill")
        }
    }
}

// Display a Token.
impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

// Display a TokenType
impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}