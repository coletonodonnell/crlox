use crate::expression::{Expr, ExprVisitor};
use crate::token::{Literal, Token, TokenType};

pub struct Interpreter;

impl Interpreter {
    fn is_truthy(literal: Literal) -> bool {
        match literal {
            Literal::Nill => return false,
            Literal::False => return false,
            Literal::True => return true,
            _ => return true
        }
    }
    
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
}

// See ExprVisitor at Expression for implementations
impl ExprVisitor<Literal> for Interpreter {
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
                if let (Literal::Num(a), Literal::Num(b)) = (left.clone(), right.clone()) {
                    match !Self::is_equal(left, right) {
                        true => return Literal::True,
                        false => return Literal::False
                    }
                } else {
                    panic!("{:?} and {:?} must both be numbers...", left, right)
                }
            }
            TokenType::EqualEqual => {
                if let (Literal::Num(a), Literal::Num(b)) = (left.clone(), right.clone()) {
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

    fn visit_grouping(&mut self, expression: Box<Expr>) -> Literal {
        return self.visit(*expression)
    }

    fn visit_literal(&mut self, value: Literal) -> Literal {
        return value;
    }

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

}