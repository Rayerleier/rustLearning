pub fn add(left: usize, right: usize) -> usize {
    left + right
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
pub fn add_two(a: usize) -> usize {
    a + 2
}

pub fn greeting_contains_name(name: &str) -> String {
    format!("Hello, {}", name)
}

pub fn greeting() -> String{
    format!("Hello!")
}

pub struct Guess{
    value: i32,
}

impl Guess{
    pub fn new(value: i32) -> Guess{
        if value<1 || value>100{
            panic!("Guess value must be between 1 and 100, got {}", value);
        }
        Guess{value}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_not_equal() {
        assert_ne!(add_two(2), 5);
    }

    #[test]
    #[should_panic]
    fn another() {
        panic!("Make this test fail");
    }

    #[test]
    fn greeting_contains_name_right() {
        let result = greeting_contains_name("Carol");
        assert!(result.contains("Carol"));
    }

    #[test]
    #[should_panic]
    fn greeting_contains_name_with_error() {
        let result = greeting();
        assert!(result.contains("Carol"),
        "Greeting did not contain name, value was `{result}`");
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        }; 
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }
    
    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }
}
