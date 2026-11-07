use std::char;

use clap::Args;

use crate::cli::commands::args::calculate_args::Token::{Agruppator, Operator, Value};
#[derive(PartialEq)]
pub enum Types {
    Number,
    Operator,
    Agruppator,
}
#[derive(Debug, PartialEq)]
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
                let category = CalculateArgs::get_category(&buffer);
                let char_category = CalculateArgs::get_category(char.encode_utf8(&mut [0; 4]));
                if category != char_category {
                    if let Some(token) = CalculateArgs::process_token(&mut buffer) {
                        tokens.push(token);
                    }
                    buffer.clear();
                    buffer.push(char);
                } else {
                    buffer.push(char);
                }
            }
            if !buffer.is_empty()
                && let Some(token) = CalculateArgs::process_token(&mut buffer)
            {
                tokens.push(token);
            }
            println!("Tokenizer version: {:?}", tokens);
            println!("Operation: {}", equation);
            println!("Result: {}", 20)
        }
        if self.equation.is_none() {
            println!("You need to input a operation.")
        }
    }
    fn get_category(s: &str) -> Option<Types> {
        match s {
            _ if s.parse::<i32>().is_ok() => Some(Types::Number),
            "(" | ")" => Some(Types::Agruppator),
            "" => None,
            _ => Some(Types::Operator),
        }
    }
    // podria sacar la validacion de categoría
    fn process_token(buffer: &mut String) -> Option<Token> {
        let category = CalculateArgs::get_category(buffer);
        if matches!(category, Some(Types::Number)) && !buffer.is_empty() {
            if let Ok(val) = buffer.parse::<i32>() {
                buffer.clear();
                return Some(Value(val));
            }
            return None;
        }
        if matches!(category, Some(Types::Operator)) {
            return Some(Operator(String::from("+")));
        }
        if matches!(category, Some(Types::Agruppator)) {
            return Some(Agruppator('('));
        }
        None
    }
}
