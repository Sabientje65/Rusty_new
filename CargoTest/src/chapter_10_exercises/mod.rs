use std::fmt::{Display, Formatter};

struct NumberSet {
    numbers: Vec<u32>
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

fn find_largest_number<T>(list: &[T]) -> &T where T: core::cmp::Ord  {
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
}