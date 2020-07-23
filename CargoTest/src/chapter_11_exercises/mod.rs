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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works(){
        assert_eq!(5 + 5, 10)
    }

    // #[test]
    // fn should_not_panic(){
    //     panic!()
    // }
}
