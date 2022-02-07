use crate::expression::{Expr, ExprVisitor};
use crate::token::{Literal, Token, TokenType};

#[derive(Default)]
pub struct Interpreter;

impl Interpreter {
    // Return trues (Nill and False are false, anything else true)
    fn is_truthy(literal: Literal) -> bool {
        match literal {
            Literal::Nill => return false,
            Literal::False => return false,
            Literal::True => return true,
            _ => return true
        }
    }
    
    // Check equality
    fn is_equal(left: Literal, right: Literal) -> bool {
        match (left, right) {
            (Literal::Nill, Literal::Nill) => true,
            (Literal::True, Literal::True) => true,
            (Literal::False, Literal::False) => true,
            (Literal::Str(a), Literal::Str(b)) => a == b,
            (Literal::Num(a), Literal::Num(b)) => a == b,
            _ => false
        }
    }

    // Convert literal to proper strings for display
    fn stringify(literal: Literal) -> String {
        match literal {
            Literal::Nill => return String::from("nill"),
            Literal::Num(a) => {
                let a = a.to_string();
                if a.ends_with(".0") {
                    return a[..a.len() - 2].to_string()
                } else {
                    return a
                }
            }
            _ => {
                return literal.to_string()
            }
        }
    }
}

// See ExprVisitor at Expression for implementation requirements
impl ExprVisitor<Literal> for Interpreter {
    // Evaluate a binary expression
    fn visit_binary(&mut self, left: Box<Expr>, operator: Token, right: Box<Expr>) -> Literal {
        let left = self.visit(*left);
        let right = self.visit(*right);

        match operator.token_type {
            TokenType::Minus => {
                if let (Literal::Num(a), Literal::Num(b)) = (left.clone(), right.clone()) {
                    return Literal::Num(a-b)
                } else {
                    panic!("{:?} and {:?} must both be numbers...", left, right)
                }
            },
            TokenType::Plus => {
                if let (Literal::Num(a), Literal::Num(b)) = (left.clone(), right.clone()) {
                    return Literal::Num(a+b)
                } else {
                    panic!("{:?} and {:?} must both be numbers...", left, right)
                }
            }
            TokenType::Slash => {
                if let (Literal::Num(a), Literal::Num(b)) = (left.clone(), right.clone()) {
                    return Literal::Num(a/b)
                } else {
                    panic!("{:?} and {:?} must both be numbers...", left, right)
                }
            },
            TokenType::Star => {
                if let (Literal::Num(a), Literal::Num(b)) = (left.clone(), right.clone()) {
                    return Literal::Num(a*b)
                } else {
                    panic!("{:?} and {:?} must both be numbers...", left, right)
                }
            },
            TokenType::Greater => {
                if let (Literal::Num(a), Literal::Num(b)) = (left.clone(), right.clone()) {
                    if a > b {
                        return Literal::True
                    } else {
                        return Literal::False
                    }
                } else {
                    panic!("{:?} and {:?} must both be numbers...", left, right)
                }
            },
            TokenType::GreaterEqual => {
                if let (Literal::Num(a), Literal::Num(b)) = (left.clone(), right.clone()) {
                    if a >= b {
                        return Literal::True
                    } else {
                        return Literal::False
                    }
                } else {
                    panic!("{:?} and {:?} must both be numbers...", left, right)
                }
            },
            TokenType::Less => {
                if let (Literal::Num(a), Literal::Num(b)) = (left.clone(), right.clone()) {
                    if a < b {
                        return Literal::True
                    } else {
                        return Literal::False
                    }
                } else {
                    panic!("{:?} and {:?} must both be numbers...", left, right)
                }
            },
            TokenType::LessEqual => {
                if let (Literal::Num(a), Literal::Num(b)) = (left.clone(), right.clone()) {
                    if a <= b {
                        return Literal::True
                    } else {
                        return Literal::False
                    }
                } else {
                    panic!("{:?} and {:?} must both be numbers...", left, right)
                }
            },
            TokenType::BangEqual => {
                if let (Literal::Num(_), Literal::Num(_)) = (left.clone(), right.clone()) {
                    match !Self::is_equal(left, right) {
                        true => return Literal::True,
                        false => return Literal::False
                    }
                } else {
                    panic!("{:?} and {:?} must both be numbers...", left, right)
                }
            }
            TokenType::EqualEqual => {
                if let (Literal::Num(_), Literal::Num(_)) = (left.clone(), right.clone()) {
                    match Self::is_equal(left, right) {
                        true => return Literal::True,
                        false => return Literal::False
                    }
                } else {
                    panic!("{:?} and {:?} must both be numbers...", left, right)
                }
            }
            _ => return Literal::Nill
        }
    }

    // Evaluate a grouping expression
    fn visit_grouping(&mut self, expression: Box<Expr>) -> Literal {
        return self.visit(*expression)
    }

    // Evaluate a literal expression
    fn visit_literal(&mut self, value: Literal) -> Literal {
        return value;
    }

    // Evaluate a unary expression
    fn visit_unary(&mut self, operator: Token, right: Box<Expr>) -> Literal {
        let right = self.visit(*right);
        match operator.token_type {
            // If the right value is a negative, take the negative of the number
            TokenType::Minus => {
                match right {
                    Literal::Num(n) => return Literal::Num(-n),
                    _ => panic!("{:?} Must be a number...", right)
                }
            }

            // If the right value is a bang, take the opposite of the boolean value
            TokenType::Bang => {
                match !Self::is_truthy(right) {
                    true => return Literal::True,
                    false => return Literal::False
                }
            }
            // Else just return nill
            _ => {
                return Literal::Nill
            }
        }
    }

    // Interpret an expression
    fn interpret(&mut self, expression: Box<Expr>) -> String {
        let value: Literal = Self::visit(self, *expression);
        return Self::stringify(value);
    }
}