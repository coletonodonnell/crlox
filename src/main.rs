use std::env;
use std::fs;
use text_io::read;
mod interpreter;
mod token;
mod scanner;
mod expression;
mod parser;
mod stmt;
mod environment;
use self::token::{Token, TokenType};
use self::parser::{Parser};
use self::interpreter::{Interpreter};
use self::stmt::Stmt;

/* 
Overview

Program flow:
Run from prompt or file -> Create Lox instance -> Scan and tokenize -> 
Parse tokens and create expressions -> Interpret tokens and execute

Scanner:
A scanner consists of an input source, the source converted into chars, the tokens, as well as
the position in the source. From here we go through and tokenize, finding certain key words,
differentiating numbers and strings, etc. There is absolutely no processing being done to 
simplify the expressions, all we are doing is finding out what the source is saying in an easier
way.

Parser:
The parser turns these tokens into statements. Tokens consist of the token type, the 
lexeme (what it is in the source,) an optional literal, and the line the token appears on. An
expression takes our token (or Literal) and then gives it context. The parser's goal is to take
the relative tokens and determine context, such as equality, comparisons, grouping, statements, etc.

Interpreter:
Now that we have a set of statements, we can now understand context. Mind you, throughout all these
steps we have ruled out syntax errors and are now only left with run time errors. These errors are 
anything that happen at run time, eg. when the program is already running. Because of how basic 
Lox is, some type errors could be determined earlier. The interpreter takes these statements and 
simplifies, it actually does the operations. This means that the Interpreter is what makes Lox, Lox.
It is the part that runs all our code we've scanned and parsed.
*/

#[derive(Default, Copy, Clone)]
pub struct Lox {
    had_error: bool,
    had_runtime_error: bool
} 

impl Lox {
    // Call error message for scanner
    pub fn scanner_error(&mut self, line: u32, message: &str) {
        if !self.had_error {
            self.had_error = true;
        }
        
        Self::report(self, line, "", message);
    }

    // Call error message for parser
    pub fn parser_error(&mut self, token: Token, message: &str) {
        if !self.had_error {
            self.had_error = true;
        }

        match token.token_type {
            TokenType::Eof => {
                Self::report(self, token.line, " at end", message)
            }
            _ => {
                Self::report(self, token.line, &*format!(" at '{}'", token.lexeme), message);
            }
        }
    }

    // Call error message for interpreter, also known as run time error.
    pub fn interpreter_error(&mut self, token: Token, message: &str) {
        if !self.had_runtime_error {
            self.had_runtime_error = true;
        }

        match token.token_type {
            TokenType::Eof => {
                Self::report(self, token.line, " at end", message)
            }
            _ => {
                Self::report(self, token.line, &*format!(" at '{}'", token.lexeme), message);
            }
        }
    }

    // Report an error as a formated error message
    fn report(&self, line: u32, where_is: &str, message: &str) {
        eprintln!("[line {}] Error{}: {}", line, where_is, message);
    }

    // Called when running from a file
    fn run_file(&mut self, path: &str) {
        // Read that juicy file as a string.
        let data: String = fs::read_to_string(path).expect("Failed To Resolve File!");

        // Run the scanner 
        Self::run(self.clone(), data);
        if self.had_error {
            std::process::exit(65); // Exit on scanner/parser error.
        } else if self.had_runtime_error {
            std::process::exit(70); // Exit on interpreter error.
        }
    }

    // This seems to create a new line for each iteration?? Need to fix...
    fn run_prompt(&mut self) {
        loop {
            let line: String = read!("{}\n");
            if line == "quit" {break;}
            Self::run(self.clone(), line);
            self.had_error = false;
        }
    }

    // Run the scanner, debug statements included
    fn run(self, input: String) {
        let mut a: scanner::Scanner = scanner::scanner_builder(self, input);
        let tokens: Vec<token::Token> = a.scan_tokens();
        // println!("Scanner:");
        // for i in tokens.clone() {
        //     println!("{}", i);
        // }
        let mut parser: Parser = Parser::parser_builder(tokens, self);
        let statements: Option<Vec<Stmt>> = parser.parse();
        match statements {
            Some(a) => {
                let mut inter: Interpreter = Interpreter::build_interpreter(self);
                inter.interpret(a);
            }
            None => {
                return
            }
        }
    }
}

fn main() {
    // Args, including initial command. If ran as binary, this should under normal circumstances 
    // be length 2 if running from file, and length 1 if running from prompt. 
    let args: Vec<String> = env::args().collect();
    
    // Check for valid command usage
    if args.len() > 2 {
        println!("Usage: crlox [optional script]");
        std::process::exit(64); // Exit
    } else if args.len() == 2 {
        // Grab reference to the second argument
        let path = &args[1];

        // Declare a mutable instance of Lox 
        let mut a = Lox{had_error: false, had_runtime_error: false};

        // Run the Lox File
        a.run_file(path);
    } else {
        // Declare a mutable instance of Lox
        let mut a = Lox{had_error: false, had_runtime_error: false};

        // Run Lox as a Prompt
        a.run_prompt();
    }
}