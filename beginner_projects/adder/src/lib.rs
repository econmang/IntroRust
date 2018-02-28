//Note: the [#should_panic] trait allows us to make sure a test case fails (hence passing the test)
//but they can be inaccurate as sometimes, if your test fails for another reason, the program
//will only register that it panicked, thereby stating it passed the test

#[derive(Debug)]
pub struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { length: 8, width: 7 };
        let smaller = Rectangle { length: 5, width: 1 };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    #[ignore]
    fn controlling_failure_messages() {
        assert!("Bananas".contains("Oranges"), "Result did not contain Oranges. Value was {}", "Bananas");
    }
    
    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle { length: 8, width: 7 };
        let smaller = Rectangle { length: 5, width: 1 };

        assert!(!smaller.can_hold(&larger));
    }

    #[test] //This attribute indicates this is a test function
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    #[ignore] //ignored unless specifically called
    fn another() {
        panic!("Make this test fail");
    }
}
