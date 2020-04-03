pub fn add (a: i32, b: i32) -> i32 {
    a + b
}

pub fn subtraction (a: i32, b: i32) -> i32 {
    a - b
}

pub fn fibonacci (a: u32) -> u32 {
    if a != 0 && a != 1 {
        return fibonacci(a - 1) + fibonacci(a - 2);
    }
    a
}

pub fn gcd(a: u32, b: u32) -> u32 {
    let gcd_result: u32;

    match a >= b {
        true =>
            match a % b == 0 {
                true => gcd_result = b,
                _ => return gcd(b, a - b),
        },
        _ => return gcd(b, a),
    }
  gcd_result
}

fn main() {
    println!("Welcome To Rust And TDD");
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        assert_eq!(add(1,2), 3)
    }

    #[test]
    fn test_adding_negative_numbers() {
       assert_eq!(add(-1,-4), -5)
    }

    #[test]
    fn test_additing_positive_and_negative() {
        assert_eq!(add(1,-2), -1);
        assert_eq!(add(2, -1), 1)
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

    #[test]
    fn test_gcd_of_multiples() {
        assert_eq!(gcd(4, 2), 2);
    }

    #[test]
    fn test_gcd_order_of_inputs_should_be_immaterial() {
        assert_eq!(gcd(4, 12), 4);
    }

    #[test]
    fn test_gcd_of_number_not_multiples_0f_each_other() {
        assert_eq!(gcd(12, 8), 4);
    }

    #[test]
    fn test_gcd_if_the_input_data_are_same() {
        assert_eq!(gcd(45, 45), 45);
    }

    #[test]
    fn test_gcd_of_number_not_multiples_0f_each_other_x() {
        assert_eq!(gcd(45, 47), 1);
    }

    #[test]
    fn test_gcd_order_of_inputs_should_be_immaterial_and_input_data_are_not_multiples() {
        assert_eq!(gcd(8, 12), 4);
    }
}


