use crate::token::{Token, TokenType, Literal};
use crate::expression::{Expr};
use crate::stmt::Stmt;

pub struct Parser {
    pub tokens: Vec<Token>,
    pub current: i32,
    pub instance: crate::Lox
}

impl Parser {
    // Return Previous Token
    fn previous(&mut self) -> Token {
        let a: Vec<Token> = self.tokens.clone();
        return a.into_iter().nth((self.current - 1) as usize).expect("Missing Element");
    }
    
    // Return Current Token
    fn peek(&mut self) -> Token {
        let a: Vec<Token> = self.tokens.clone();
        return a.into_iter().nth(self.current as usize).expect("Missing Element");
    }

    // Are we at the end of the line? If so, it has been fun partner...
    fn is_end(&mut self) -> bool {
        return matches!(Self::peek(self).token_type, TokenType::Eof);
    }

    // Consume a token
    fn advance(&mut self) -> Token {
        if !Self::is_end(self) {
            self.current += 1;
        }
        return Self::previous(self);
    }

    // Check if current "tokens" TokenType equals requested TokenType "token_type"
    fn check(&mut self, token_type: TokenType) -> bool {
        if Self::is_end(self) {
            return false;
        } else {
            return Self::peek(self).token_type == token_type;
        }
    }

    // If current token matches any of the Vector of TokenTypes "types," then consume a token and return true, otherwise return false
    fn match_type(&mut self, types: Vec<TokenType>) -> bool {
        for a in types {
            if Self::check(self, a) {
                Self::advance(self);
                return true;
            }
        }

        return false;
    }

    // If there is an error send it here to report to the Lox instance
    fn error(&mut self, token: Token, message: String) {
        self.instance.parser_error(token, &*message);
    }

    // Check to see if ) exists
    fn consume(&mut self, token_type: TokenType, message: String) -> Result<Token, String> {
        if Self::check(self, token_type) {
            return Ok(Self::advance(self));
        }
        let token: Token = Self::peek(self);
        Self::error(self, token, message.clone());
        return Err(message);
    }

    // primary → NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" ;
    fn primary(&mut self) -> Expr {
        if Self::match_type(self, vec![TokenType::False]) {
            return Expr::Literal {value: Literal::False}
        }

        if Self::match_type(self, vec![TokenType::True]) {
            return Expr::Literal {value: Literal::True}
        }

        if Self::match_type(self, vec![TokenType::Nil]) {
            return Expr::Literal {value: Literal::Nill}
        }

        if Self::match_type(self, vec![TokenType::Num]) {
            return Expr::Literal {value: Self::previous(self).literal.unwrap()}
        }

        if Self::match_type(self, vec![TokenType::String]) {
            return Expr::Literal {value: Self::previous(self).literal.unwrap()}
        }

        if Self::match_type(self, vec![TokenType::LParen]) {
            let expr: Expr = Self::expression(self);
            let _a: Result<Token, String> = Self::consume(self, TokenType::RParen, "Expect ')' after expression.".to_string());
            return Expr::Grouping {expression: Box::new(expr)};
        } else {
            let a: Token = Self::peek(self);
            Self::error(self, a, "Expect expression.".to_string());
            return Expr::Literal {value: Literal::Str("Placeholder".to_string())};
        }
    }

    // comma → primary ( comma primary )*
    fn comma(&mut self) -> Expr {
        let mut expr: Expr = Self::primary(self);

        while Self::match_type(self, vec![TokenType::Comma]) {
            let operator: Token = Self::previous(self);
            let right: Expr = Self::primary(self);
            expr = Expr::Binary {left: Box::new(expr), operator: operator, right: Box::new(right)}
        }
        return expr;
    }

    // unary → ( "!" | "-" ) unary | comma 
    fn unary(&mut self) -> Expr {
        if Self::match_type(self, vec![TokenType::Bang, TokenType::Minus]) {
            let operator: Token = Self::previous(self);
            let right: Expr = Self::unary(self);
            return Expr::Unary {operator: operator, right: Box::new(right)};
        }

        return Self::comma(self);
    }

    // factor → unary ( ( "/" | "*" ) unary )*
    fn factor(&mut self) -> Expr {
        let mut expr: Expr = Self::unary(self);

        while Self::match_type(self, vec![TokenType::Slash, TokenType::Star]) {
            let operator: Token = Self::previous(self);
            let right: Expr = Self::unary(self);
            expr = Expr::Binary {left: Box::new(expr), operator: operator, right: Box::new(right)}
        }
        return expr;
    }

    // term → factor ( ( "-" | "+" ) factor )* ;
    fn term(&mut self) -> Expr {
        let mut expr: Expr = Self::factor(self);

        while Self::match_type(self, vec![TokenType::Minus, TokenType::Plus]) {
            let operator: Token = Self::previous(self);
            let right: Expr = Self::factor(self);
            expr = Expr::Binary {left: Box::new(expr), operator: operator, right: Box::new(right)}
        }
        return expr;
    }

    // comparison → term ( ( ">" | ">=" | "<" | "<=" ) term )*
    fn comparison(&mut self) -> Expr {
        let mut expr: Expr = Self::term(self);
        while Self::match_type(self, vec![TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual]) {
            let operator: Token = Self::previous(self);
            let right: Expr = Self::term(self);
            expr = Expr::Binary {left: Box::new(expr), operator: operator, right: Box::new(right)};

        }
        return expr;
    }

    // equality → comparison ( ( "!=" | "==" ) comparison )*
    fn equality(&mut self) -> Expr {
        let mut expr: Expr = Self::comparison(self);
        while Self::match_type(self, vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator: Token = Self::previous(self);
            let right: Expr = Self::comparison(self);
            expr = Expr::Binary {left: Box::new(expr), operator: operator, right: Box::new(right)};
        }
        return expr;
    }

    // expression → equality
    fn expression(&mut self) -> Expr {
        return Self::equality(self);
    }

    fn expression_statement(&mut self) -> Stmt {
        let value: Expr = self.expression();
        let _a: Result<Token, String> = self.consume(TokenType::Semicolon, "Expect ';' after value.".to_string());
        return Stmt::Expression{ expression: value };
    }

    fn print_statement(&mut self) -> Stmt {
        let value: Expr = self.expression();
        let _a: Result<Token, String> = self.consume(TokenType::Semicolon, "Expect ';' after value.".to_string());
        return Stmt::Print{ expression: value };
    }

    fn statement(&mut self) -> Stmt {
        if self.match_type(vec![TokenType::Print]) {
            return self.print_statement();
        }

        return self.expression_statement();
    }

    pub fn parser_builder(tokens: Vec<Token>, instance: crate::Lox) -> Parser {
        return Parser {
            tokens: tokens,
            current: 0,
            instance: instance
        }
    }

    pub fn parse(&mut self) -> Option<Vec<Stmt>>{
        let mut statements: Vec<Stmt> = Vec::new();

        while !self.is_end() {
            statements.push(self.statement());
        }

        if !self.instance.had_error {
            return Some(statements)
        } else {
            return None
        }
    }
}