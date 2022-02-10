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
        return matches!(self.peek().token_type, TokenType::Eof);
    }

    // Consume a token
    fn advance(&mut self) -> Token {
        if !self.is_end() {
            self.current += 1;
        }

        return self.previous();
    }

    // Check if current "tokens" TokenType equals requested TokenType "token_type"
    fn check(&mut self, token_type: TokenType) -> bool {
        if self.is_end() {
            return false;
        } else {
            return self.peek().token_type == token_type;
        }
    }

    // If current token matches any of the Vector of TokenTypes "types," then consume a token and return true, otherwise return false
    fn match_type(&mut self, types: Vec<TokenType>) -> bool {
        for a in types {
            if self.check(a) {
                self.advance();
                return true;
            }
        }

        return false;
    }

    // If there is an error send it here to report to the Lox instance
    fn error(&mut self, token: Token, message: String) {
        self.instance.parser_error(token, &*message);
    }

    fn consume(&mut self, token_type: TokenType, message: String) -> Option<Token> {
        if self.check(token_type) {
            return Some(self.advance());
        }

        let token: Token = self.peek();
        self.error(token, message.clone());
        return None;
    }

    // primary → NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" ;
    fn primary(&mut self) -> Expr {
        if self.match_type(vec![TokenType::False]) {
            return Expr::Literal {value: Literal::False}
        }

        if self.match_type(vec![TokenType::True]) {
            return Expr::Literal {value: Literal::True}
        }

        if self.match_type(vec![TokenType::Nil]) {
            return Expr::Literal {value: Literal::Nill}
        }

        if self.match_type(vec![TokenType::Num]) {
            return Expr::Literal {value: self.previous().literal.unwrap()}
        }

        if self.match_type(vec![TokenType::String]) {
            return Expr::Literal {value: self.previous().literal.unwrap()}
        }

        if self.match_type(vec![TokenType::Id]) {
            return Expr::Variable {token: self.previous()};
        }

        if self.match_type(vec![TokenType::LParen]) {
            let expr: Expr = self.expression();
            let _a: Option<Token> = self.consume(TokenType::RParen, "Expect ')' after expression.".to_string());
            return Expr::Grouping {expression: Box::new(expr)};

        } else {
            let a: Token = self.peek();
            self.error(a, "Expect expression.".to_string());
            return Expr::Literal {value: Literal::Str("Placeholder".to_string())};
        }
    }

    // comma → primary ( comma primary )*
    fn comma(&mut self) -> Expr {
        let mut expr: Expr = self.primary();

        while self.match_type(vec![TokenType::Comma]) {
            let operator: Token = self.previous();
            let right: Expr = self.primary();
            expr = Expr::Binary {left: Box::new(expr), operator: operator, right: Box::new(right)}
        }

        return expr;
    }

    // unary → ( "!" | "-" ) unary | comma 
    fn unary(&mut self) -> Expr {
        if self.match_type(vec![TokenType::Bang, TokenType::Minus]) {
            let operator: Token = self.previous();
            let right: Expr = self.unary();
            return Expr::Unary {operator: operator, right: Box::new(right)};
        }

        return self.comma();
    }

    // factor → unary ( ( "/" | "*" ) unary )*
    fn factor(&mut self) -> Expr {
        let mut expr: Expr = self.unary();

        while self.match_type(vec![TokenType::Slash, TokenType::Star]) {
            let operator: Token = self.previous();
            let right: Expr = self.unary();
            expr = Expr::Binary {left: Box::new(expr), operator: operator, right: Box::new(right)}
        }

        return expr;
    }

    // term → factor ( ( "-" | "+" ) factor )* ;
    fn term(&mut self) -> Expr {
        let mut expr: Expr = self.factor();

        while self.match_type(vec![TokenType::Minus, TokenType::Plus]) {
            let operator: Token = self.previous();
            let right: Expr = self.factor();
            expr = Expr::Binary {left: Box::new(expr), operator: operator, right: Box::new(right)}
        }

        return expr;
    }

    // comparison → term ( ( ">" | ">=" | "<" | "<=" ) term )*
    fn comparison(&mut self) -> Expr {
        let mut expr: Expr = self.term();
        while self.match_type(vec![TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual]) {
            let operator: Token = self.previous();
            let right: Expr = self.term();
            expr = Expr::Binary {left: Box::new(expr), operator: operator, right: Box::new(right)};
        }

        return expr;
    }

    // equality → comparison ( ( "!=" | "==" ) comparison )*
    fn equality(&mut self) -> Expr {
        let mut expr: Expr = self.comparison();
        while self.match_type(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator: Token = self.previous();
            let right: Expr = self.comparison();
            expr = Expr::Binary {left: Box::new(expr), operator: operator, right: Box::new(right)};
        }
        return expr;
    }

    // expression → equality
    fn expression(&mut self) -> Expr {
        return self.equality();
    }

    fn expression_statement(&mut self) -> Stmt {
        let value: Expr = self.expression();
        let _a: Option<Token> = self.consume(TokenType::Semicolon, "Expect ';' after value.".to_string());
        return Stmt::Expression{ expression: value };
    }

    fn print_statement(&mut self) -> Stmt {
        let value: Expr = self.expression();
        let _a: Option<Token> = self.consume(TokenType::Semicolon, "Expect ';' after value.".to_string());
        return Stmt::Print{ expression: value };
    }

    fn statement(&mut self) -> Stmt {
        if self.match_type(vec![TokenType::Print]) {
            return self.print_statement();
        }

        return self.expression_statement();
    }

    fn synchronize(&mut self) {
        todo!()
    }

    fn var_declaration(&mut self) -> Option<Stmt> {
        let name: Option<Token> = self.consume(TokenType::Id, "Expect variable name.".to_string());
        match name {
            Some(a) => {
                let mut initalizer: Option<Expr> = None;
                if self.match_type(vec![TokenType::Equal]) {
                    initalizer = Some(self.expression());
                }

                let _a: Option<Token> = self.consume(TokenType::Semicolon, "Expect ';' after variable decleration.".to_string());
                return Some(Stmt::Var{name: a, right: initalizer})
            },
            None => return None
        }
    }

    fn declaration(&mut self) -> Option<Stmt> {
        if self.match_type(vec![TokenType::Var]) {
            match self.var_declaration() {
                Some(a) => return Some(a),
                None => {
                    self.synchronize();
                    return None
                }
            }
        } else {
            return Some(self.statement())
        }
    }

    pub fn parse(&mut self) -> Option<Vec<Stmt>>{
        let mut statements: Vec<Stmt> = Vec::new();

        while !self.is_end() {
            match self.declaration() {
                Some(a) => statements.push(a),
                None => {}
            }
        }

        if !self.instance.had_error {
            return Some(statements)
        } else {
            return None
        }
    }

    pub fn parser_builder(tokens: Vec<Token>, instance: crate::Lox) -> Parser {
        return Parser {
            tokens: tokens,
            current: 0,
            instance: instance
        }
    }
}