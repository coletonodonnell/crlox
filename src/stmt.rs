use crate::expression::Expr;
use crate::token::Token;

#[derive(Clone, Debug)]
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
    },
    If {
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>
    },
    While {
        condition: Expr,
        body: Box<Stmt>
    }
}

pub trait StmtVisitor<> {
    fn execute(&mut self, stmt: Stmt) {
        match stmt {
            Stmt::Expression {expression: a} => self.visit_expression(a),
            Stmt::Print {expression: a} => self.visit_print(a),
            Stmt::Var {name: a, right: b} => self.visit_var(a, b),
            Stmt::Block {statements: a} => self.visit_block(a),
            Stmt::If {condition: a, then_branch: b, else_branch: c} => self.visit_if(a, b, c),
            Stmt::While {condition: a, body: b} => self.visit_while(a, b),
        }
    }

    fn visit_expression(&mut self, expression: Expr);
    fn visit_print(&mut self, expression: Expr);
    fn visit_var(&mut self, name: Token, right: Option<Expr>);
    fn visit_block(&mut self, statements: Vec<Stmt>);
    fn visit_if(&mut self, condition: Expr, then_branch: Box<Stmt>, else_branch: Option<Box<Stmt>>);
    fn visit_while(&mut self, condition: Expr, body: Box<Stmt>);
}