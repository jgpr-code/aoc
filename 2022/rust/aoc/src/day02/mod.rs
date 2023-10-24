pub fn say_hello() {
    println!("Hello!");
    say_goodbye();
}

pub fn return5() -> i32 {
    5
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
}
