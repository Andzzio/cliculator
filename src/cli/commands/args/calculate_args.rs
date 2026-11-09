use std::mem::take;

use clap::Args;

use crate::cli::commands::args::{
    calculate_args::Token::{Operator, Value},
    operators::{Associativity, OPERATORS},
    token::Token,
    types::Types::{self},
};
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
            let tokens_post_order = CalculateArgs::tokens_to_post_order(&tokens);
            let result = CalculateArgs::get_result(&tokens_post_order);
            println!("Tokenizer version: {:?}", tokens);
            println!("Post-Order version: {:?}", tokens_post_order);
            println!("Operation: {}", equation);
            println!("Result: {}", result)
        }
        if self.equation.is_none() {
            println!("You need to input a operation.")
        }
    }
    fn get_result(post_order_tokens: &[Token]) -> f64 {
        let mut result: Vec<f64> = Vec::new();
        for token in post_order_tokens {
            match token {
                Token::Value(v) => {
                    result.push(*v);
                }
                Token::Operator(o) => match (result.pop(), result.pop()) {
                    (Some(b), Some(a)) => {
                        if let Some(op_info) = OPERATORS.get(o) {
                            let operation = (op_info.apply)(a, b);
                            result.push(operation);
                        }
                    }
                    _ => eprintln!("need more operators"),
                },
                Token::Agruppator(_) => {}
            }
        }
        if let Some(result) = result.pop() {
            return result;
        }
        -1.0
    }
    fn tokens_to_post_order(tokens: &[Token]) -> Vec<Token> {
        let mut output: Vec<Token> = Vec::new();
        let mut operators_stack: Vec<Token> = Vec::new();
        //println!("===DEBUG===");
        //println!("initial output: {:?}", output);
        //println!("initial operators_stack: {:?}", operators_stack);
        //println!("============");
        for token in tokens {
            //println!("current token: {:?}", token);
            //println!("output: {:?}", output);
            //println!("operators_stack: {:?}", operators_stack);
            //println!("========");
            match token {
                Token::Value(n) => {
                    output.push(Value(*n));
                }
                Token::Operator(o) => {
                    while let Some(token) = operators_stack.last() {
                        if let Operator(token) = token
                            && let (Some(op_info_top), Some(op_info_current)) =
                                (OPERATORS.get(token), OPERATORS.get(o))
                        {
                            if op_info_current.associativity == Associativity::Left {
                                if op_info_top.precedence >= op_info_current.precedence
                                    && let Some(op_top) = operators_stack.pop()
                                {
                                    output.push(op_top);
                                } else {
                                    break;
                                }
                            } else if op_info_current.associativity == Associativity::Right {
                                if op_info_top.precedence > op_info_current.precedence
                                    && let Some(op_top) = operators_stack.pop()
                                {
                                    output.push(op_top);
                                } else {
                                    break;
                                }
                            }
                        }
                    }
                    operators_stack.push(Token::Operator(o.clone()));
                }
                Token::Agruppator(a) => {
                    output.push(Token::Agruppator(*a));
                }
            }
        }
        while operators_stack.last().is_some() {
            if let Some(operator) = operators_stack.pop() {
                output.push(operator);
            }
        }
        output
    }
    fn get_category(s: &str) -> Option<Types> {
        match s {
            _ if s.parse::<i32>().is_ok() => Some(Types::Number),
            "(" | ")" => Some(Types::Agruppator),
            "" => None,
            _ => Some(Types::Operator),
        }
    }
    //TODO: What if: ((10 - 5) + 10) -> ((
    //TODO What if: (5 + 5 -> )?
    fn process_token(buffer: &mut String) -> Option<Token> {
        let category = CalculateArgs::get_category(buffer);
        if matches!(category, Some(Types::Number)) && !buffer.is_empty() {
            if let Ok(val) = buffer.parse::<f64>() {
                buffer.clear();
                return Some(Value(val));
            }
            return None;
        }
        if matches!(category, Some(Types::Operator)) {
            return Some(Operator(take(buffer)));
        }
        if matches!(category, Some(Types::Agruppator)) {
            return buffer.chars().next().map(Token::Agruppator);
        }
        None
    }
}
