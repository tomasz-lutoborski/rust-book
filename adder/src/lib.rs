#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
} 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            height: 10,
            width: 5
        };
        
        let smaller = Rectangle {
            height: 4,
            width: 2
        };

        assert!(larger.can_hold(&smaller));
    }
}
