use clap::Args;
pub enum Token {
    Value(i32),
    Operator(String),
    Agruppator(char),
}
#[derive(Args)]
pub struct CalculateArgs {
    pub execute: Option<String>,
}
impl CalculateArgs {
    pub fn run(&self) {
        // "20 + 1" -trim-> "20+1" -?-> [20, +, 1]
        let mut tokens: Vec<Token> = Vec::new();
        if let Some(name) = &self.execute {
            let name = name.trim().replace(" ", "");
            println!("Operation: {}", name);
            println!("Result: {}", 20)
        }
        if self.execute.is_none() {
            println!("You need to input a operation.")
        }
    }
}
