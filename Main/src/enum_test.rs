use std::borrow::{BorrowMut, Borrow};
use std::ops::{Add, AddAssign};

struct Position {
    x: i32,
    y: i32
}

enum Message {
    Quit,
    Move(Position),
    Write(String),
    ChangeColor { r: i32, g: i32, b: i32 }
}

enum Integer<'l> {
    One(Option<&'l i32>),
    Two(Option<&'l i32>)
}

impl Integer<'_> {
    fn multiply(&self) -> Option<&'_ i32>{
        let value = match self {
            Integer::One(i) => i, 
            Integer::Two(i) => i
        };
        
        match value {
            Some(i) => {
                Integer::multiply_number(i, self);
                Some(i)
            },
            _ => None
        }
    }
    
    fn multiply_number(mut num: &'_ i32, by: &Integer) {
        match by {
            Integer::One(_) => {},
            Integer::Two(_) => { 
                // num.add_assign(num);
            }
        }
    }
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => {},
            Message::Move(pos) => { }
            Message::Write(message) => {}
            Message::ChangeColor { r, g, b } => {}
        }
        
        // match self {
        //     Message::Quit => { },
        //     Message::Move(Position) => {  }
        // }
    }
}

// struct QuitMessage;
// struct MoveMessage {
//     position: Position
// }
// struct WriteMessage(String);
// struct ChangeColor { r: i32, g: i32, b: i32 };

pub fn enum_test(){
    let x: i32 = 5;
    // let y: Option<i8> = Some(5);
    
    // let sum = x + y.unwrap();
    
    let two = Integer::Two(Some(&x));
    
    two.multiply();
    
    println!("Final value: {}", match two.multiply() {
        Some(v) => v.to_string(),
        None => String::from("NOTHING")
    })
}