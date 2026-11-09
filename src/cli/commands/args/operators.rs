use std::{collections::HashMap, sync::LazyLock};

#[derive(PartialEq)]
pub enum Associativity {
    Left,
    Right,
}
pub struct OpInfo {
    pub precedence: u8,
    pub associativity: Associativity,
    pub apply: fn(f64, f64) -> f64,
}
pub static OPERATORS: LazyLock<HashMap<String, OpInfo>> = LazyLock::new(|| {
    HashMap::from([
        (
            String::from("+"),
            OpInfo {
                precedence: 2,
                associativity: Associativity::Left,
                apply: |a, b| a + b,
            },
        ),
        (
            String::from("-"),
            OpInfo {
                precedence: 2,
                associativity: Associativity::Left,
                apply: |a, b| a - b,
            },
        ),
        (
            String::from("^"),
            OpInfo {
                precedence: 4,
                associativity: Associativity::Right,
                apply: |a, b| a.powf(b),
            },
        ),
    ])
});
