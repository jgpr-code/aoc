pub fn return5() -> i32 {
    5
}

pub fn return6() -> i32 {
    6
}

pub fn say_hello() {
    println!("Hello!");
    say_goodbye();
}

fn say_goodbye() {
    println!("Goodbye!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_return5() {
        assert_eq!(return5(), 5);
    }

    #[test]
    fn test_return6() {
        assert_eq!(return6(), 6);
    }
}
