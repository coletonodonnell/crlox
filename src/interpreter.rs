use crate::expression::{Expr, ExprVisitor};
use crate::token::{Literal, Token};

pub struct Interpreter;

impl ExprVisitor<Literal> for Interpreter {
    fn visit_binary(&mut self, left: Box<Expr>, operator: Token, right: Box<Expr>) -> Literal {
        todo!()
    }

    fn visit_grouping(&mut self, expression: Box<Expr>) -> Literal {
        todo!()
    }

    fn visit_literal(&mut self, value: Literal) -> Literal {
        todo!()
    }

    fn visit_unary(&mut self, operator: Token, right: Box<Expr>) -> Literal {
        todo!()
    }

}