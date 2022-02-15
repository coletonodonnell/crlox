use crate::expression::{Expr, ExprVisitor};
use crate::token::{Literal, Token, TokenType};
use crate::stmt::{Stmt, StmtVisitor};
use crate::environment::Environment;

pub struct Interpreter {
    instance: crate::Lox,
    environment: Environment
}

impl Interpreter {
    // Build an interpreter
    pub fn build_interpreter(instance: crate::Lox) -> Interpreter {
        Interpreter {
            instance: instance,
            environment: Environment::build_envrionment(instance)
        }
    }

    // If there is an error send it here to report to the Lox instance
    fn error(&mut self, token: Token, message: String) {
        self.instance.interpreter_error(token, &*message);
    }

    fn binary_error(&mut self, left: Literal, operator: Token, right: Literal) {
        let message: String = format!("{:?} and {:?} must both be numbers", left, right);
        self.error(operator, message);
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

    // Interpret an expression
    pub fn interpret(&mut self, statements: Vec<Stmt>) {
        for statement in statements {
            self.execute(statement)
        }
    }
}

impl StmtVisitor<()> for Interpreter {
    fn visit_expression(&mut self, expression: Expr) {
        self.visit(expression);
    }

    fn visit_print(&mut self, expression: Expr) {
        let value: Option<Literal> = self.visit(expression);
        match value {
            Some(a) => {
                println!("{}", Self::stringify(a))
            }
            None => {
                return
            }
        }
    }

    fn visit_var(&mut self, name: Token, right: Option<Expr>) {
        let value: Option<Literal>;
        match right {
            Some(a) => {
                value = self.visit(a);
                match value {
                    Some(b) => self.environment.define(name.lexeme, b),
                    _ => {}
                }
            }
            None => {
                self.environment.define(name.lexeme, Literal::Nill)
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
            // Subtraction
            TokenType::Minus => {
                if let (Literal::Num(a), Literal::Num(b)) = (left.clone(), right.clone()) {
                    return Some(Literal::Num(a-b))
                } else {
                    self.binary_error(left, operator, right);
                    return None
                }
            },

            // Addition
            TokenType::Plus => {
                match (left.clone(), right.clone()) {
                    // String Concatenation (with and without numbers)
                    (Literal::Num(a), Literal::Str(b)) => {
                        return Some(Literal::Str(format!("{}{}", a, b)))
                    },
                    (Literal::Str(a), Literal::Num(b)) => {
                        return Some(Literal::Str(format!("{}{}", a, b)))
                    },
                    (Literal::Str(a), Literal::Str(b)) => {
                        return Some(Literal::Str(format!("{}{}", a, b)))
                    },
                    // Actual Addition
                    (Literal::Num(a), Literal::Num(b)) => {
                        return Some(Literal::Num(a+b))
                    },
                    (a, b) => {
                        self.error(operator, format!("{} and {} must be either a String or a Num", a, b));
                        return None
                    }
                }
            },

            // Division
            TokenType::Slash => {
                if let (Literal::Num(a), Literal::Num(b)) = (left.clone(), right.clone()) {
                    // Check if we are dividing by 0
                    if b == 0.0 {
                        self.error(operator, "Can't divide by 0".to_string());
                        return None
                    }
                    return Some(Literal::Num(a/b))
                } else {
                    self.binary_error(left, operator, right);
                    return None
                }
            },

            // Multiplication
            TokenType::Star => {
                if let (Literal::Num(a), Literal::Num(b)) = (left.clone(), right.clone()) {
                    return Some(Literal::Num(a*b))
                } else {
                    self.binary_error(left, operator, right);
                    return None
                }
            },

            // Check if a is greater than b
            TokenType::Greater => {
                if let (Literal::Num(a), Literal::Num(b)) = (left.clone(), right.clone()) {
                    if a > b {
                        return Some(Literal::True)
                    } else {
                        return Some(Literal::False)
                    }
                } else {
                    self.binary_error(left, operator, right);
                    return None
                }
            },

            // Check if a is greater than or equal to b
            TokenType::GreaterEqual => {
                if let (Literal::Num(a), Literal::Num(b)) = (left.clone(), right.clone()) {
                    if a >= b {
                        return Some(Literal::True)
                    } else {
                        return Some(Literal::False)
                    }
                } else {
                    self.binary_error(left, operator, right);
                    return None
                }
            },

            // Check if a is less than b
            TokenType::Less => {
                if let (Literal::Num(a), Literal::Num(b)) = (left.clone(), right.clone()) {
                    if a < b {
                        return Some(Literal::True)
                    } else {
                        return Some(Literal::False)
                    }
                } else {
                    self.binary_error(left, operator, right);
                    return None
                }
            },

            // Check if a is less than or equal to b
            TokenType::LessEqual => {
                if let (Literal::Num(a), Literal::Num(b)) = (left.clone(), right.clone()) {
                    if a <= b {
                        return Some(Literal::True)
                    } else {
                        return Some(Literal::False)
                    }
                } else {
                    self.binary_error(left, operator, right);
                    return None
                }
            },

            // Check if a is not equal to b
            TokenType::BangEqual => {
                if let (Literal::Num(_), Literal::Num(_)) = (left.clone(), right.clone()) {
                    match !Self::is_equal(left, right) {
                        true => return Some(Literal::True),
                        false => return Some(Literal::False)
                    }
                } else {
                    self.binary_error(left, operator, right);
                    return None
                }
            },

            // Check if a is equal to b
            TokenType::EqualEqual => {
                if let (Literal::Num(_), Literal::Num(_)) = (left.clone(), right.clone()) {
                    match Self::is_equal(left, right) {
                        true => return Some(Literal::True),
                        false => return Some(Literal::False)
                    }
                } else {
                    self.binary_error(left, operator, right);
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
                        self.error(operator, message);
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

    fn visit_variable(&mut self, token: Token) -> Option<Literal> {
        match self.environment.get(token.clone()) {
            Ok(a) => return Some(a),
            Err(a) => {
                self.error(token, a);
                return None
            }
        }
    }

    fn visit_assignment(&mut self, name: Token, value: Box<Expr>) -> Option<Literal> {
        let literal = self.visit(*value);
        self.environment.assign(name, literal.clone().unwrap());
        return literal
    }
}