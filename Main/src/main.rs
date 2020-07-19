mod enum_test;
use enum_test::{enum_test};

// use std::net::{IpAddr};

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

enum IpAddress {
    V4(u8, u8, u8, u8),
    V6(String)
}



impl Rectangle {
    fn calculate_area(&self) -> u32{
        // self = Rectangle {
        //     width: self.width,
        //     height: self.height
        // }
        
        self.width * self.height
    }
    
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.height > other.height && self.width > other.width
    }
    
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }
}

fn slice_test(s: &String) -> Option<String> {
    let bytes = s.as_bytes();
    let iterator = bytes.iter().enumerate();
    
    // let idx: Option<usize> = None;
    
    

    let idx2: Option<usize> = {
        let mut result: Option<usize> = None;

        // loop {
        //     match bytes.iter().next() {
        //         Some(value) => {
        //             value.
        //         },
        //         None => None
        //     }
        // }
        
        for (i, &item) in iterator {
            if item ==  b' '{
                println!("Matched!");
                result = Some(i);
                break;
            }
        }
        
        // None
        result

        // println!("Returning: {}", match result {
        //     Some(value) => value.to_string(),
        //     None => "NONE"
        // });
        // result
    };


    
    match idx2 {
        Some(idx) => Some(String::from(&s[0..idx])),
        None => None
    }
}

fn main() {
    enum_test();
    
    let rect = Rectangle {
        width: 1,
        height: 2
    };
    
    let rect2 = Rectangle {
        ..rect
    };
    
    let rect2 = retake_ownership(rect2);
    
    let p = rect2;
    
    {
        let rect3 = rect;
        calculate_area(rect3);
        // let y = rect3.height;
    }
    
    // let x = rect.height;
    
    match slice_test(&String::from("testje abc def")) {
        Some(value) => println!("{}", value),
        None => println!("Hoe dan")
    }
    
    let square = Rectangle::square(32);
    
    let thingy = IpAddress::V4(5, 6, 7, 8);
    
    
    match thingy{
        IpAddress::V4(x, y, r, _) => {
            let p = x + y + r;
            String::from("abc");
        },
        IpAddress::V6(str) => {
            
        }
    }
    
    println!("{}", p.calculate_area())
}

fn calculate_area(dimensions: Rectangle) -> u32 {
    dimensions.width * dimensions.height
}
fn retake_ownership(rect: Rectangle) -> Rectangle { rect }
