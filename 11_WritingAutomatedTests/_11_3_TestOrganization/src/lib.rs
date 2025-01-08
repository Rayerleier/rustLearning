pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn add_two(a: usize) -> usize {
    a + 2
}

// allow tests to access private functions
fn internal_adder(a: usize, b: usize) -> usize {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}