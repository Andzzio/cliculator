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
        // "20 + 1" -trim-> "20+1" -?-> [20, +, 1]
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
    fn process_token(char: char, buffer: &mut String) -> Option<Token> {
        let is_number: bool = char.is_ascii_digit();
        let is_agruppator: bool = char == '(' || char == ')';
        let is_operator: bool = !is_number && !is_agruppator;
        if is_number {
            return Some(Value(10));
        }
        if is_operator {
            return Some(Operator(String::from("+")));
        }
        if is_agruppator {
            return Some(Agruppator('('));
        }
        None
    }
}
