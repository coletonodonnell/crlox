use crate::expression::Expr;

pub enum Stmt {
    Expression {
        expression: Expr
    },
    Print {
        expression: Expr
    }
}

pub trait StmtVisitor<T> {
    fn execute(&mut self, stmt: Stmt) -> T {
        match stmt {
            Stmt::Expression {expression: a} => self.visit_expression(a),
            Stmt::Print {expression: a} => self.visit_print(a)
        }
    }

    fn visit_expression(&mut self, expression: Expr) -> T;
    fn visit_print(&mut self, expression: Expr) -> T;
}