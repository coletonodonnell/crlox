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
    fn match_type_vec(&mut self, types: Vec<TokenType>) -> bool {
        for a in types {
            if self.check(a) {
                self.advance();
                return true;
            }
        }

        return false;
    }

    // If current token matches token_type, then consume a token and return true, otherwise return false
    fn match_type(&mut self, token_type: TokenType) -> bool {
        if self.check(token_type) {
            self.advance();
            return true;
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
        if self.match_type(TokenType::False) {
            return Expr::Literal {value: Literal::False}
        }

        if self.match_type(TokenType::True) {
            return Expr::Literal {value: Literal::True}
        }

        if self.match_type(TokenType::Nil) {
            return Expr::Literal {value: Literal::Nill}
        }

        if self.match_type(TokenType::Num) {
            return Expr::Literal {value: self.previous().literal.unwrap()}
        }

        if self.match_type(TokenType::String) {
            return Expr::Literal {value: self.previous().literal.unwrap()}
        }

        if self.match_type(TokenType::Id) {
            return Expr::Variable {token: self.previous()};
        }

        if self.match_type(TokenType::LParen) {
            let expr: Expr = self.expression();
            let _: Option<Token> = self.consume(TokenType::RParen, "Expect ')' after expression.".to_string());
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

        while self.match_type(TokenType::Comma) {
            let operator: Token = self.previous();
            let right: Expr = self.primary();
            expr = Expr::Binary {left: Box::new(expr), operator: operator, right: Box::new(right)}
        }

        return expr;
    }

    // unary → ( "!" | "-" ) unary | comma 
    fn unary(&mut self) -> Expr {
        if self.match_type_vec(vec![TokenType::Bang, TokenType::Minus]) {
            let operator: Token = self.previous();
            let right: Expr = self.unary();
            return Expr::Unary {operator: operator, right: Box::new(right)};
        }

        return self.comma();
    }

    // factor → unary ( ( "/" | "*" ) unary )*
    fn factor(&mut self) -> Expr {
        let mut expr: Expr = self.unary();

        while self.match_type_vec(vec![TokenType::Slash, TokenType::Star]) {
            let operator: Token = self.previous();
            let right: Expr = self.unary();
            expr = Expr::Binary {left: Box::new(expr), operator: operator, right: Box::new(right)}
        }

        return expr;
    }

    // term → factor ( ( "-" | "+" ) factor )* ;
    fn term(&mut self) -> Expr {
        let mut expr: Expr = self.factor();

        while self.match_type_vec(vec![TokenType::Minus, TokenType::Plus]) {
            let operator: Token = self.previous();
            let right: Expr = self.factor();
            expr = Expr::Binary {left: Box::new(expr), operator: operator, right: Box::new(right)}
        }

        return expr;
    }

    // comparison → term ( ( ">" | ">=" | "<" | "<=" ) term )*
    fn comparison(&mut self) -> Expr {
        let mut expr: Expr = self.term();
        while self.match_type_vec(vec![TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual]) {
            let operator: Token = self.previous();
            let right: Expr = self.term();
            expr = Expr::Binary {left: Box::new(expr), operator: operator, right: Box::new(right)};
        }

        return expr;
    }

    // equality → comparison ( ( "!=" | "==" ) comparison )*
    fn equality(&mut self) -> Expr {
        let mut expr: Expr = self.comparison();
        while self.match_type_vec(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator: Token = self.previous();
            let right: Expr = self.comparison();
            expr = Expr::Binary {left: Box::new(expr), operator: operator, right: Box::new(right)};
        }
        return expr;
    }

    // logic_nd → equality ( "and" equality )*
    fn logic_nd(&mut self) -> Expr {
        let mut expr: Expr = self.equality();

        while self.match_type(TokenType::And) {
            let operator: Token = self.previous();
            let right: Expr = self.equality();
            expr = Expr::Logical{ left: Box::new(expr), operator: operator, right: Box::new(right) };
        }

        return expr;
    }

    // logic_or → logic_nd ( "or" logic_nd )*
    fn logic_or(&mut self) -> Expr {
        let mut expr: Expr = self.logic_nd();

        while self.match_type(TokenType::Or) {
            let operator: Token = self.previous();
            let right: Expr = self.logic_nd();
            expr = Expr::Logical{ left: Box::new(expr), operator: operator, right: Box::new(right) };
        }
        return expr;
    }

    // assignment → IDENTIFIER "=" assignment | logic_or
    fn assignment(&mut self) -> Expr {
        let expr: Expr = self.logic_or();

        if self.match_type(TokenType::Equal) {
            let equals: Token = self.previous();
            let value: Expr = self.assignment();

            match expr {
                Expr::Variable{token} => {
                    return Expr::Assign{name: token, value: Box::new(value)}
                }
                _ => {
                    self.error(equals, "Invalid assignment target.".to_string());
                }
            }
        }

        return expr;
    }

    // expression → assignment
    fn expression(&mut self) -> Expr {
        return self.assignment();
    }

    // Process an expression statement and return it as a new Expression Stmt.
    fn expression_statement(&mut self) -> Stmt {
        let value: Expr = self.expression();
        let _: Option<Token> = self.consume(TokenType::Semicolon, "Expect ';' after value.".to_string());
        return Stmt::Expression{ expression: value };
    }

    fn for_statement(&mut self) -> Stmt {
        self.consume(TokenType::LParen, "Expect '(' after 'for'.".to_string());

        let initializer: Option<Stmt>;
        if self.match_type(TokenType::Semicolon) {
            initializer = None;
        } else if self.match_type(TokenType::Var) {
            initializer = self.var_declaration();
        } else {
            initializer = Some(self.expression_statement());
        }

        let condition: Expr;
        if !self.check(TokenType::Semicolon) {
            condition = self.expression();
        } else {
            condition = Expr::Literal{value: Literal::True};
        }
        self.consume(TokenType::Semicolon, "Expect ';' after loop condition.".to_string());

        let increment: Option<Expr>;
        if !self.check(TokenType::RParen) {
            increment = Some(self.expression())
        } else {
            increment = None;
        }
        self.consume(TokenType::RParen, "Expect ')' after for clauses.".to_string());

        let mut body: Stmt = self.statement();

        if increment.is_some() {
            body = Stmt::Block{
                statements: vec![body, Stmt::Expression{ expression: increment.unwrap() }]
            }
        }
        body = Stmt::While{ condition: condition, body: Box::new(body) };

        if initializer.is_some() {
            body = Stmt::Block{ statements: vec![initializer.unwrap(), body] }
        }

        return body;
    }

    fn while_statement(&mut self) -> Stmt {
        let _ = self.consume(TokenType::LParen, "Expect '(' after 'while'.".to_string());
        let condition: Expr = self.expression();
        let _ = self.consume(TokenType::RParen, "Expect ')' after 'while'.".to_string());
        let body: Stmt = self.statement();

        return Stmt::While{ condition: condition, body: Box::new(body)}
    }

    fn if_statement(&mut self) -> Stmt {
        let _ = self.consume(TokenType::LParen, "Expect '(' after 'if'.".to_string());
        let condition: Expr = self.expression();
        let _ = self.consume(TokenType::RParen, "Expect ')' after if condition".to_string());

        let then_branch: Stmt = self.statement();
        let mut else_branch: Option<Box<Stmt>> = None;

        if self.match_type(TokenType::Else) {
            else_branch = Some(Box::new(self.statement()));
        }

        return Stmt::If {condition: condition, then_branch: Box::new(then_branch), else_branch: else_branch}
    }

    fn print_statement(&mut self) -> Stmt {
        let value: Expr = self.expression();
        let _: Option<Token> = self.consume(TokenType::Semicolon, "Expect ';' after value.".to_string());
        return Stmt::Print{ expression: value };
    }

    fn block(&mut self) -> Vec<Stmt>{
        let mut statements: Vec<Stmt> = Vec::new();
        
        while !self.check(TokenType::RBrace) && !self.is_end() {
            if let Some(a) = self.declaration() {
                statements.push(a);
            }
        }

        self.consume(TokenType::RBrace, "Expect '}' after block.".to_string());
        return statements;
    }

    fn statement(&mut self) -> Stmt {
        if self.match_type(TokenType::For) {
            return self.for_statement();
        }
        if self.match_type(TokenType::If) {
            return self.if_statement();
        }
        if self.match_type(TokenType::Print) {
            return self.print_statement();
        }
        if self.match_type(TokenType::While) {
            return self.while_statement();
        }
        if self.match_type(TokenType::LBrace) {
            return Stmt::Block{statements: self.block()};
        }

        return self.expression_statement();
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_end() {
            if !(self.previous().token_type == TokenType::Semicolon) {
                match self.peek().token_type {
                    TokenType::Class | TokenType::Fun | TokenType::Var | TokenType::For | TokenType::If | TokenType::While | TokenType::Print | TokenType::Return => {
                        return;
                    }
                    _ => {}
                }
            } else {
                return
            }
        }
    }

    fn var_declaration(&mut self) -> Option<Stmt> {
        let name: Option<Token> = self.consume(TokenType::Id, "Expect variable name.".to_string());
        match name {
            Some(a) => {
                let mut initializer: Option<Expr> = None;
                if self.match_type(TokenType::Equal) {
                    initializer = Some(self.expression());
                }

                let _: Option<Token> = self.consume(TokenType::Semicolon, "Expect ';' after variable declaration.".to_string());
                return Some(Stmt::Var{name: a, right: initializer})
            },
            None => return None
        }
    }

    fn declaration(&mut self) -> Option<Stmt> {
        if self.match_type(TokenType::Var) {
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