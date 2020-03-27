pub fn add (a: i32, b: i32) -> i32 {
    a + b
}

fn subtraction (a: i32, b: i32) -> i32 {
   a - b
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
}


