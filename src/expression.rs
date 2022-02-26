use crate::token::{Token, Literal};

#[derive(Clone, Debug)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>
    },
    Grouping {
        expression: Box<Expr>
    },
    Literal {
        value: Literal
    },
    Unary {
        operator: Token,
        right: Box<Expr>
    },
    Variable {
        token: Token
    },
    Assign {
        name: Token,
        value: Box<Expr>
    },
    Logical {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>
    },
    Call {
        callee: Box<Expr>,
        paren: Token,
        arguments: Vec<Expr> 
    }
}

pub trait ExprVisitor<T> {
    fn visit(&mut self, expr: Expr) -> Option<T> {
        match expr {
            Expr::Binary {left, operator, right } => self.visit_binary(left, operator, right),
            Expr::Grouping {expression} => self.visit_grouping(expression),
            Expr::Literal {value} => self.visit_literal(value),
            Expr::Unary {operator, right} => self.visit_unary(operator, right),
            Expr::Variable {token} => self.visit_variable(token),
            Expr::Assign {name, value} => self.visit_assignment(name, value),
            Expr::Logical {left, operator, right} => self.visit_logical(left, operator, right),
            Expr::Call {callee, paren, arguments} => self.visit_call(callee, paren, arguments)
        }
    }

    fn visit_binary(&mut self, left: Box<Expr>, operator: Token, right: Box<Expr>) -> Option<T>;
    fn visit_grouping(&mut self, expression: Box<Expr>) -> Option<T>;
    fn visit_literal(&mut self, value: Literal) -> Option<T>;
    fn visit_unary(&mut self, operator: Token, right: Box<Expr>) -> Option<T>;
    fn visit_variable(&mut self, token: Token) -> Option<T>;
    fn visit_assignment(&mut self, name: Token, value: Box<Expr>) -> Option<T>;
    fn visit_logical(&mut self, left: Box<Expr>, operator: Token, right: Box<Expr>) -> Option<T>;
    fn visit_call(&mut self, callee: Box<Expr>, paren: Token, arguments: Vec<Expr>) -> Option<T>;
}