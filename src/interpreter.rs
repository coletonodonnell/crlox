use crate::expression::{Expr, ExprVisitor};
use crate::token::{Literal, Token, TokenType};

pub struct Interpreter {
    instance: crate::Lox
}

impl Interpreter {
    // Build an interpreter
    pub fn build_interpreter(instance: crate::Lox) -> Interpreter {
        Interpreter {
            instance: instance
        }
    }

    // If there is an error send it here to report to the Lox instance
    fn error(&mut self, token: Token, message: String) {
        self.instance.interpreter_error(token, &*message);
    }

    fn binary_error(&mut self, left: Literal, operator: Token, right: Literal) {
        let message: String = format!("{:?} and {:?} must both be numbers...", left, right);
        Self::error(self, operator, message);
    }

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
    fn stringify(literal: Option<Literal>, expression: Box<Expr>) -> String {
        match literal {
            Some(a) => {
                match a {
                    Literal::Nill => return String::from("nill"),
                    Literal::Num(b) => {
                        let b = b.to_string();
                        if b.ends_with(".0") {
                            return b[..b.len() - 2].to_string()
                        } else {
                            return b
                        }
                    }
                    _ => {
                        return a.to_string()
                    }
                }
            }
            None => {
                Expr::show(*expression)
            }
        }
    }
}

// See ExprVisitor at Expression for implementation requirements
impl ExprVisitor<Literal> for Interpreter {
    // Evaluate a binary expression
    fn visit_binary(&mut self, b_left: Box<Expr>, operator: Token, b_right: Box<Expr>) -> Option<Literal> {
        let v_left = self.visit(*b_left);
        let v_right = self.visit(*b_right);

        let left: Literal;
        let right: Literal;
        match (v_left, v_right) {
            (Some(a), Some(b)) => {
                left = a;
                right = b;
            }
            _ => {
                return None
            }
        }

        match operator.token_type {
            TokenType::Minus => {
                if let (Literal::Num(a), Literal::Num(b)) = (left.clone(), right.clone()) {
                    return Some(Literal::Num(a-b))
                } else {
                    Self::binary_error(self, left, operator, right);
                    return None
                }
            },

            TokenType::Plus => {
                if let (Literal::Num(a), Literal::Num(b)) = (left.clone(), right.clone()) {
                    return Some(Literal::Num(a+b))
                } else {
                    Self::binary_error(self, left, operator, right);
                    return None
                }
            },

            TokenType::Slash => {
                if let (Literal::Num(a), Literal::Num(b)) = (left.clone(), right.clone()) {
                    return Some(Literal::Num(a/b))
                } else {
                    Self::binary_error(self, left, operator, right);
                    return None
                }
            },

            TokenType::Star => {
                if let (Literal::Num(a), Literal::Num(b)) = (left.clone(), right.clone()) {
                    return Some(Literal::Num(a*b))
                } else {
                    Self::binary_error(self, left, operator, right);
                    return None
                }
            },

            TokenType::Greater => {
                if let (Literal::Num(a), Literal::Num(b)) = (left.clone(), right.clone()) {
                    if a > b {
                        return Some(Literal::True)
                    } else {
                        return Some(Literal::False)
                    }
                } else {
                    Self::binary_error(self, left, operator, right);
                    return None
                }
            },

            TokenType::GreaterEqual => {
                if let (Literal::Num(a), Literal::Num(b)) = (left.clone(), right.clone()) {
                    if a >= b {
                        return Some(Literal::True)
                    } else {
                        return Some(Literal::False)
                    }
                } else {
                    Self::binary_error(self, left, operator, right);
                    return None
                }
            },

            TokenType::Less => {
                if let (Literal::Num(a), Literal::Num(b)) = (left.clone(), right.clone()) {
                    if a < b {
                        return Some(Literal::True)
                    } else {
                        return Some(Literal::False)
                    }
                } else {
                    Self::binary_error(self, left, operator, right);
                    return None
                }
            },

            TokenType::LessEqual => {
                if let (Literal::Num(a), Literal::Num(b)) = (left.clone(), right.clone()) {
                    if a <= b {
                        return Some(Literal::True)
                    } else {
                        return Some(Literal::False)
                    }
                } else {
                    Self::binary_error(self, left, operator, right);
                    return None
                }
            },

            TokenType::BangEqual => {
                if let (Literal::Num(_), Literal::Num(_)) = (left.clone(), right.clone()) {
                    match !Self::is_equal(left, right) {
                        true => return Some(Literal::True),
                        false => return Some(Literal::False)
                    }
                } else {
                    Self::binary_error(self, left, operator, right);
                    return None
                }
            },

            TokenType::EqualEqual => {
                if let (Literal::Num(_), Literal::Num(_)) = (left.clone(), right.clone()) {
                    match Self::is_equal(left, right) {
                        true => return Some(Literal::True),
                        false => return Some(Literal::False)
                    }
                } else {
                    Self::binary_error(self, left, operator, right);
                    return None
                }
            },
            _ => return Some(Literal::Nill)
        }
    }

    // Evaluate a grouping expression
    fn visit_grouping(&mut self, expression: Box<Expr>) -> Option<Literal> {
        return self.visit(*expression)
    }

    // Evaluate a literal expression
    fn visit_literal(&mut self, value: Literal) -> Option<Literal> {
        return Some(value)
    }

    // Evaluate a unary expression
    fn visit_unary(&mut self, operator: Token, b_right: Box<Expr>) -> Option<Literal> {
        let v_right = self.visit(*b_right);

        let right: Literal;
        match v_right {
            Some(a) => {
                right = a;
            }
            _ => {
                return None
            }
        }
        match operator.token_type {
            // If the right value is a negative, take the negative of the number
            TokenType::Minus => {
                match right {
                    Literal::Num(n) => return Some(Literal::Num(-n)),
                    _ => {
                        let message = format!("{:?} Must be a number...", right);
                        Self::error(self, operator, message);
                        return None
                    }
                }
            }

            // If the right value is a bang, take the opposite of the boolean value
            TokenType::Bang => {
                match !Self::is_truthy(right) {
                    true => return Some(Literal::True),
                    false => return Some(Literal::False)
                }
            }
            // Else just return nill
            _ => {
                return Some(Literal::Nill)
            }
        }
    }

    // Interpret an expression
    fn interpret(&mut self, expression: Box<Expr>) -> String {
        let value: Option<Literal> = Self::visit(self, *expression.clone());
        return Self::stringify(value, expression);
    }
}