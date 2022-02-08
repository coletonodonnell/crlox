use crate::token::{Token, Literal};

#[derive(Clone)]
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
}

impl Expr {
    // Pretty Print an Expr
    // TODO: Implement this show using visitor instead of this garbage.
    pub fn show(expr: Expr) -> String {
        // Match expr, as we require different steps to generate a string depending on the type
        match expr {
            Expr::Binary {left, operator, right} => {
                return format!("({} {} {})", operator.lexeme, Expr::show(*left), Expr::show(*right));
            },
            Expr::Grouping {expression} => {
                return format!("(group {})", Expr::show(*expression));
            },
            Expr::Unary {operator, right} => {
                return format!("({} {})", operator.lexeme, Expr::show(*right));
            }
            Expr::Literal {value} => {
                match value {
                    Literal::Str(a) => {
                        return a;
                    },
                    Literal::Num(a) => {
                        return a.to_string();
                    },
                    Literal::False => {
                        return "false".to_string();
                    },
                    Literal::True => {
                        return "true".to_string();
                    },
                    Literal::Nill => {
                        return "nill".to_string();
                    }
                }
                
            }
        }
    }
}

pub trait ExprVisitor<T> {
    fn visit(&mut self, expr: Expr) -> Option<T> {
        match expr {
            Expr::Binary {left, operator, right } => self.visit_binary(left, operator, right),
            Expr::Grouping {expression} => self.visit_grouping(expression),
            Expr::Literal {value} => self.visit_literal(value),
            Expr::Unary {operator, right} => self.visit_unary(operator, right),
        }
    }

    fn visit_binary(&mut self, left: Box<Expr>, operator: Token, right: Box<Expr>) -> Option<T>;
    fn visit_grouping(&mut self, expression: Box<Expr>) -> Option<T>;
    fn visit_literal(&mut self, value: Literal) -> Option<T>;
    fn visit_unary(&mut self, operator: Token, right: Box<Expr>) -> Option<T>;
    fn interpret(&mut self, expression: Box<Expr>) -> String;
}