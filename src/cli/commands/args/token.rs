#[derive(Debug, PartialEq)]
pub enum Token {
    Value(f64),
    Operator(String),
    Agruppator(char),
}
