use std::fmt::{Display, Formatter};

struct NumberSet {
    numbers: Vec<u32>
}

struct Point<T, U> {
    x: T,
    y: U
}

trait PrettyPrint {
    fn pretty_print(&self) -> String;
}

fn write_summary<T>(t: &T) where T : PrettyPrint {
    print!("{}", t.pretty_print())
}

impl<T, U> PrettyPrint for Point<T, U> where 
    T : Display,
    U : Display 
{
    fn pretty_print(&self) -> String {
        format!("{} - {}", self.x, self.y)
    }
}

impl<T> Point<u32, T> {
    fn get_x(&self) -> u32 {
        self.x
    }
}

impl NumberSet {
    fn largest(&self) -> u32 {
        find_largest_number(&self.numbers).to_owned()
    }
}

impl Display for NumberSet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let numbers = &self.numbers;
        let numbers_len = numbers.len();
        let mut idx = 0;
        
        loop {
            if idx >= numbers_len{
                break;  
            }
            
            let n = numbers[idx];
            write!(f, "{}", n);

            idx += 1;
            
            if idx < numbers_len {
                write!(f,  ",  ");
            }
        }
        
        Result::Ok(())
    }
}

fn find_largest_number<T>(list: &[T]) -> &T where T: core::cmp::PartialOrd  {
    let mut largest: &T = &list[0];

    for item in list {
        if item > largest {
            largest = item;   
        }
    }
    
    largest
}

pub fn run(){
    let set = NumberSet {
        numbers: vec![5, 6, 9, 7, 8]
    };
    
    println!("From set: {}, the largest value is: {}", set, set.largest());
    
    let p1: Point<u32, _> = Point { x: 1, y: 2.43 };
    let p2: Point<u64, _> = Point { x: 1, y: 2.43 };
    
    
    let x = p1.get_x();

    write_summary(&p1)
}