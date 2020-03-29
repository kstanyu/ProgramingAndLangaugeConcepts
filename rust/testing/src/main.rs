pub fn add (a: i32, b: i32) -> i32 {
    a + b
}

fn subtraction (a: i32, b: i32) -> i32 {
    a - b
}

fn fibonacci (a: u32) -> u32 {
    if a != 0 && a != 1 {
        print!("im the value of a { }", a);
        return fibonacci(a - 1) + fibonacci(a - 2);
    }
    a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        assert_eq!(add(1,2), 3)
    }

    #[test]
    fn test_subtraction() {
        assert_eq!(subtraction(1,2), -1)
    }

    #[test]
    fn test_fibonacci_sequence_first_term() {
        assert_eq!(fibonacci(0), 0);
    }
    #[test]
    fn test_fibonacci_sequence_second_term() {
        assert_eq!(fibonacci(1), 1);
    }

    #[test]
    fn test_fibonacci_sequence_third_term() {
        assert_eq!(fibonacci(2), 1);
    }

    #[test]
    fn test_fibonacci_sequence_nth_term() {
        assert_eq!(fibonacci(3), 2);
    }
}


