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
    },
    Block {
        statements: Vec<Stmt>
    }
}

pub trait StmtVisitor<> {
    fn execute(&mut self, stmt: Stmt) {
        match stmt {
            Stmt::Expression {expression: a} => self.visit_expression(a),
            Stmt::Print {expression: a} => self.visit_print(a),
            Stmt::Var {name: a, right: b} => self.visit_var(a, b),
            Stmt::Block {statements: a} => self.visit_block(a)
        }
    }

    fn visit_expression(&mut self, expression: Expr);
    fn visit_print(&mut self, expression: Expr);
    fn visit_var(&mut self, name: Token, right: Option<Expr>);
    fn visit_block(&mut self, statements: Vec<Stmt>);
}