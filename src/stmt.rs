use crate::expression::Expr;
use crate::token::Token;

pub enum Stmt {
    Expression {
        expression: Expr
    },
    Print {
        expression: Expr
    },
    Var {
        name: Token,
        right: Option<Expr>,
    }
}

pub trait StmtVisitor<T> {
    fn execute(&mut self, stmt: Stmt) -> T {
        match stmt {
            Stmt::Expression {expression: a} => self.visit_expression(a),
            Stmt::Print {expression: a} => self.visit_print(a),
            Stmt::Var {name: a, right: b} => self.visit_var(a, b)
        }
    }

    fn visit_expression(&mut self, expression: Expr) -> T;
    fn visit_print(&mut self, expression: Expr) -> T;
    fn visit_var(&mut self, name: Token, right: Option<Expr>) -> T;
}