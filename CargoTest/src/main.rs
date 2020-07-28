mod root_module;
mod sub_module;
mod chapter_8_exercises;
mod chapter_10_exercises;
mod chapter_11_exercises;

pub mod exercise;

use crate::front_of_house::Location;




mod front_of_house {
    pub struct Location {
        name: String
    }
    
    impl Location {
        fn print_name(&self){
            println!("{}", self.name);
        }
        
        pub fn print_name_public(&self){
            self.print_name();
            self.do_thing()
        }
        
        pub fn new(name: String) -> Location {
            Location {
                name
            }
        }
    }
    
    mod hosting {
        impl super::Location {
            pub fn do_thing(&self){
                self.print_name()
            }
        }
    }
}

mod boundary {
    use rand::Rng;
    use std::iter::FromIterator;

    pub struct User {
        pub first_name: String,
        password: String
    }
    
    pub mod sub {
        pub struct SubStruct {
            
        }
    }   
    
    impl User {
        pub fn new(first_name: String) -> User {
            User {
                first_name,
                password: generate_password()
            }
        }
        
        pub fn get_by_password(password: String) -> Option<User> {
            if password == "Password" {
                let u = User::new(String::from("Danny"));
                
                Some(u)
            }
            else { None }
        }
        
        pub fn set_password(&mut self, password: String){
            self.password = password;
        }
        
        pub fn get_scrambled_password(&self) -> String {
            crate::string_extensions::scramble(&self.password)
        }
        
        pub fn consume(self) { }
    }
    
    fn generate_password() -> String {
        String::from("Password")
    }
}

mod string_extensions {
    use rand::{Rng, AsByteSliceMut};
    use std::iter::FromIterator;
    use std::cmp::max;

    pub fn generate_random_string(len: usize) -> String {
        let mut new_string = String::with_capacity(len);
        let mut rng = rand::thread_rng();
        
        const MIN: u8 = 33;
        const MAX: u8 = 124;
        
        //Loop until capacity reached
        for _ in 0..new_string.capacity() {
            //Take a random character from the ASCII table
            let c_ascii: char = rng.gen_range(MIN, MAX) as char;
            
            //Append our character to our string
            new_string.push(c_ascii);
        }
        
        new_string
    }

    pub fn scramble_mutate_2(str: &mut String) {
        let mut result = String::with_capacity(str.len());
        let mut chars = Vec::from_iter(str.chars().into_iter());
        let mut rng = rand::thread_rng();

        loop {
            //Guard against panic raised by gen_range when having high same as low
            let current_index = match chars.len() {
                0 => 0,
                v => rng.gen_range(0, v)
            };

            //When our vector is exhausted; None will be returned
            match chars.get(current_index) {
                Some(c) => {
                    result.push(c.to_owned());
                    chars.remove(current_index);
                },
                None => break
            }
        }
        
        //Update the pointer to refer to our new string
        *str = result;
    }
    
    pub fn scramble_2(str: &String) -> String {
        let mut scrambled = String::with_capacity(str.len());
        let mut chars = Vec::from_iter(str.chars().into_iter());
        let mut rng = rand::thread_rng();

        loop {
            //Guard against panic raised by gen_range when having high same as low
            let current_index = match chars.len() {
                0 => 0,
                v => rng.gen_range(0, v)
            };

            match chars.get(current_index) {
                Some(c) => {
                    scrambled.push(c.to_owned());
                    chars.remove(current_index);
                },
                None => break
            }
        }
        
        scrambled
    }
    
    pub fn scramble(str: &String) -> String {
        //Create a new empty string
        let mut scrambled = String::with_capacity(str.len());

        //Create a vector based on our password bytes
        let mut bytes_vec: Vec<&u8> = Vec::from_iter(str.as_bytes().iter());

        // let mut iter = password_copy.iter();
        let mut rng = rand::thread_rng();

        loop {
            //Guard against panic raised by gen_range when having high same as low
            let current_index = match bytes_vec.len() {
                0 => 0,
                v => rng.gen_range(0, v)
            };

            println!("Attempting to read char at: {}", current_index);
            println!("Current length: {}", bytes_vec.len());

            match bytes_vec.get(current_index) {
                Some(byte) => {
                    //Reference -> value
                    let b_clone = byte.clone().clone();
                    let c = char::from(b_clone);
                    char::from(b_clone);

                    println!("Pushing: {} - {}", b_clone, c);

                    scrambled.push(c);
                    bytes_vec.remove(current_index);
                },
                None => break
            }
        }

        //Return our scrambled password
        scrambled
    }
}

fn test(){
    let user1 = StronkUser::get_by_password(String::from("Password"));
}

//Test hoisting
pub use self::boundary::User as StronkUser;
use self::{boundary::User};

fn run_exercises<T: exercise::exc::Exercise>(exercises: &Vec<Box<T>>){
    for exercise in exercises {
        exercise.run();
        
        // T::run(exercise);
        
        // exercise.run();
    }
}

fn add(x: u32, y: u32) -> u32{
    x + y
}

fn main() {
    /*
    test();
    
    // use boundary::sub::SubStruct;
    // SubStruct {};
    
    let user1 = User::get_by_password(String::from("Password"));
    
    // let mut us = user1.unwrap();
    // us.first_name = String::from("Test");
    
    println!("{}", match user1 {
        Some(user) => user.first_name,
        None => String::from("NOT FOUND")
    });
    
    println!("SCRAMBLED: {}", boundary::User::new(String::from("Danny")).get_scrambled_password());
    
    let mut random_string = string_extensions::generate_random_string(25);
    println!("Random string (scrambled): {}", string_extensions::scramble(&random_string));
    println!("Random string (scrambled_2): {}", string_extensions::scramble_2(&random_string));
    println!("Random string: {}, length: {}", random_string, random_string.len());
    
    string_extensions::scramble_mutate_2(&mut random_string);
    println!("Mutate scramble: {}", random_string);

    root_module::root::print_from_module();
    sub_module::other_module::other::print_from_other();
     */
    
    // chapter_8_exercises::run();
    chapter_10_exercises::run();
    
    println!("Running exercises...");

    let c11 = Box::from(chapter_11_exercises::Chapter11::new());
    let c112 = Box::from(chapter_11_exercises::Chapter112::new());
    
    //Size is not known at runtime; thus we have to use Box<> constructs
    let exercises: Vec<Box<dyn exercise::exc::Exercise>> = vec![c11, c112];

    for exercise in exercises {
        exercise.run();
    }
    
    // run_exercises(&exercises);
}
