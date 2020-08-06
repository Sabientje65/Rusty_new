use crate::exercise;
use std::thread;
use std::time::Duration;
use std::collections::HashMap;
use std::ops::Deref;

struct LazyNum<T: Fn(u32) -> u32> {
    calculation: T,
    value_map: HashMap<u32, u32>
}

impl<T: Fn(u32) -> u32> LazyNum<T> {
    fn new(calculation: T) -> LazyNum<T> {
        LazyNum {
            calculation,
            value_map: HashMap::new()
        }
    }
    
    fn value(&mut self, arg: u32) -> u32 {
        let calculation = &self.calculation;
        
        *self.value_map.entry(arg)
            .or_insert_with(|| { 
                println!("Executing fn");
                (calculation)(arg) 
            })
        
        // *self.value_map.entry(arg)
        //     .or_insert_with(|| { return (self.calculation)(arg); })
    }
}

pub fn run(){
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;
    
    generate_workout(simulated_user_specified_value, simulated_random_number);
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = LazyNum::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    
    let mut do_thing = |new_intensity: u32| {
        // println!("Today, do {} pushups!", expensive_result.value(new_intensity));
        // println!("Next, do {} situps!", expensive_result.value(new_intensity));
        
        if new_intensity < 25 {
            println!("Today, do {} pushups!", expensive_result.value(new_intensity));
            println!("Next, do {} situps!", expensive_result.value(new_intensity));
        } else {
            if random_number == 3 {
                println!("Take a break today! Remember to stay hydrated!");
            } else {
                println!("Today, run for {} minutes!", expensive_result.value(new_intensity));
            }
        }
    };

    do_thing(intensity);
    do_thing(intensity + 1);
    do_thing(intensity);
    // (&do_thing)(intensity + 1);
    // (&do_thing)(intensity);
    
    // if intensity < 25 {
    //     println!("Today, do {} pushups!", expensive_result.value(intensity));
    //     println!("Next, do {} situps!", expensive_result.value(intensity));
    // } else {
    //     if random_number == 3 {
    //         println!("Take a break today! Remember to stay hydrated!");
    //     } else {
    //         println!("Today, run for {} minutes!", expensive_result.value(intensity));
    //     }
    // }
}