#[warn(dead_code)]

// An example for calculator to learn examples
pub mod calculator {
    pub fn add(x: i32, y: i32) -> i32 {
        x + y
    }
    pub fn sub(x: i32, y: i32) -> i32 {
        x - y
    }
    pub fn multiply(x: i32, y: i32) -> i32 {
        x * y
    }
    pub fn div(x: i32, y: i32) -> i32 {
        x / y
    }
}

pub fn Usage() {
    println!("You are not even knowing how to use a calculator???")
}