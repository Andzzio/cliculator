use clap::Args;

use crate::cli::commands::args::calculate_args::Token::{Agruppator, Operator, Value};
#[derive(Debug)]
pub enum Token {
    Value(i32),
    Operator(String),
    Agruppator(char),
}
#[derive(Args)]
pub struct CalculateArgs {
    pub equation: Option<String>,
}
impl CalculateArgs {
    pub fn run(&self) {
        // "20 + 1" -trim-> "20+1" -?-> [Value(20), Operator("+"), Value(1)]
        let mut tokens: Vec<Token> = Vec::new();
        let mut buffer: String = String::new();
        if let Some(equation) = &self.equation {
            let equation = equation.trim().replace(" ", "");
            for char in equation.chars() {
                let token = CalculateArgs::process_token(char, &mut buffer);
                if let Some(token) = token {
                    tokens.push(token);
                }
            }
            println!("Tokenizer version: {:?}", tokens);
            println!("Operation: {}", equation);
            println!("Result: {}", 20)
        }
        if self.equation.is_none() {
            println!("You need to input a operation.")
        }
    }
    fn is_number(s: &str) -> bool {
        s.parse::<i32>().is_ok()
    }
    fn is_agruppator(s: &str) -> bool {
        s == "(" || s == ")"
    }
    fn is_operator(s: &str) -> bool {
        !CalculateArgs::is_number(s) && !CalculateArgs::is_agruppator(s)
    }
    fn process_token(char: char, buffer: &mut String) -> Option<Token> {
        buffer.push(char);
        if CalculateArgs::is_number(buffer) && !buffer.is_empty() {
            if let Ok(val) = buffer.parse::<i32>() {
                buffer.clear();
                return Some(Value(val));
            }
            return None;
        }
        if CalculateArgs::is_operator(buffer) {
            return Some(Operator(String::from("+")));
        }
        if CalculateArgs::is_agruppator(buffer) {
            return Some(Agruppator('('));
        }
        None
    }
}
