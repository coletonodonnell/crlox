
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
    // Printy Print an Expr
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