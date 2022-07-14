pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!("The input value is not valid!")
        }
        Guess {
            value,
        }
    }
}


// This is a unit test
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[should_panic]
    fn is_valid() {
        Guess::new(101);
    }
}

