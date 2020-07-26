use crate::exercise;

pub struct Chapter11 { }
pub struct Chapter112 { }

impl Chapter11 {
    pub fn new() -> Chapter11 {
        Chapter11 { }
    }
}

impl Chapter112 {
    pub fn new() -> Chapter112 {
        Chapter112 { }
    }
}

impl exercise::exc::Exercise for Chapter112 {
    fn run(&self) {
        println!("Running - chapter 112");
    }
}

impl exercise::exc::Exercise for Chapter11 {
    fn run(&self) {
        println!("Running - chapter 11");
    }
}

struct Rect {
    w: u32,
    h: u32
}

impl Rect {
    fn new(w: u32, h: u32) -> Rect {
        Rect { w, h }
    }
    
    fn can_hold(&self, other: &Rect) -> bool{
        return self.w > other.w && self.h > other.h;
    }
}

fn add_two(a: u32) -> u32 {
    a + 2
}
fn echo_not_something(str: &String) -> String {
    String::from("something")
}

fn panic_on_something_or_not_something(str: &String) {
    if str == "not something" || str == "something" {
        panic!("value was {}", str)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works(){
        assert_eq!(5 + 5, 10)
    }

    #[test]
    fn larger_can_hold_smaller(){
        let larger = Rect::new(10, 5);
        let smaller = Rect::new(9, 4);
        
        assert!(larger.can_hold(&smaller))
    }
    
    #[test]
    fn smaller_cannot_hold_larger(){
        let larger = Rect::new(10, 5);
        let smaller = Rect::new(9, 4);

        assert!(!smaller.can_hold(&larger))
    }
    
    #[test]
    fn it_adds_two(){
        assert_eq!(add_two(2), 4)
    }
    
    #[test]
    fn it_does_not_add_one(){
        assert_ne!(add_two(2), 3)
    }
    
    #[test]
    fn it_should_not_return_something(){
        let r1 = String::from("value");
        let something = echo_not_something(&r1);

        assert_ne!(something, "something", "Expected value \"{}\" to not be \"{}\"", something, "something")
    }

    #[test]
    #[should_panic(expected = "value was something")]
    fn value_is_something(){
        panic_on_something_or_not_something(&String::from("something"))
    }

    #[test]
    #[should_panic(expected = "value was not something")]
    fn value_is_not_something(){
        // panic_on_something_or_not_something(&String::from("something"))

        panic_on_something_or_not_something(&String::from("not something"))
    }

    #[test]
    fn with_result_ok() -> Result<(), String> {
        Ok(())
    }
    
    #[test]
    fn with_result_err() -> Result<(), String> {
        Err(String::from("We /failed/"))
    }
    
    // #[test]
    // fn should_not_panic(){
    //     panic!()
    // }
}
