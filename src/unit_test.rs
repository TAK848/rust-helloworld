struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn compare_area(&self, other: &Rectangle) -> bool {
        self.width * self.height > other.width * other.height
    }
}
fn double_value(a: i32) -> i32 {
    a * 2
}
fn greeting(name: &str) -> String {
    format!("Hello {}", name)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_double_value() {
        assert_eq!(double_value(10), 20);
    }
    #[test]
    fn test_greeting() {
        assert_eq!(greeting("John"), "Hello John");
    }
    #[test]
    fn test_compare_area() {
        let rect1 = Rectangle {
            width: 100,
            height: 20,
        };
        let rect2 = Rectangle {
            width: 20,
            height: 20,
        };
        assert!(rect1.compare_area(&rect2));
    }
}
