pub mod root {
    // use std::fmt::Pointer;

    use std::alloc::Layout;
    use std::collections::HashMap;
    use std::ptr::hash;

    struct Test {
        name: String
    }
    
    impl std::fmt::Display for Test {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Name of Test struct: {}", self.name)
        }
    }
    
    impl Test {
        pub fn string_pointer(s: &str) -> usize {
            // s.make_ascii_lowercase();
            
            s.len()
        }
    }
    
    // impl Test {
    //     fn clone(&self) -> Test {
    //         self.
    //         
    //         return Test {
    //             ..self
    //         }
    //     }
    // }
    
    pub fn print_from_module(){
        let t = Test {
            name: String::from("pronk")
        };
        
        // t.clone();
        // let t2 = Test {
        //     ..t
        // };
        
        // let  t_ref = &mut t;
        // let t2_ref = &t2;
        
        {
            let mut v: Vec<&Test> = Vec::new();
            v.push(&t);

            println!("veccie: {}", &v[0])    
        }
        
        {
            //alloc*
            let mut str = "broepie";
            Test::string_pointer(str);

            println!("{}", t);
            
            //Transfer ownership (= transfer all values from stack from t.name to x)
            //heap value remains unchanged
            //rust has no invalidated and no longer allows usage of t.name
            //this to prevent the variable from potentially going out of scope twice; causing bugs
            //this because t.name no longer owns the associated piece of memory
            //the associated piece of memory is now owned by x
            let x = t.name;
            
            //Should not cause println to crahs
            //let x = &t.name;

            const INT_LITERAL: u32 = 500_000; //_ ayyy
            
            let veccy = vec![1, 2];
            let veccy_keys = vec![String::from("s1"), String::from("s2")];
            
            //veccy and veccy_keys are both consumed by into_iter/zip
            //values are moved out of the original vectors
            //collect() can become many a type; thus we need to help Rust by typehinting to HashMap<>
            //both key and value type can be inferred and thus we can discard those via an underscore
            let mut hashy: HashMap<_, _> = veccy_keys.into_iter().zip(veccy).collect();
            
            println!("Length (should be 2): {}", hashy.len());
            
            //Insert overwrites existing values
            hashy.insert(String::from("s1"), 4);
            
            println!("Length (should be 2): {}", hashy.len());

            //Only insert when not present
            hashy.entry(String::from("s1")).or_insert(5);
            
            //Rust automagically marks our variable as mutable
            let entry_s3 = hashy.entry(String::from("s3")).or_insert(9);
            
            *entry_s3 += 1;
            
            println!("Length (should be 3): {}", hashy.len());

            // veccy_keys.push(1);
            
            //Should crash; t.name is no longer owner by t
            // println!("{}", t);

            //Create pointer
            let x2 = &x;
            
            //Kabüm
            //let y = t.name;
        }
        //free (= drop)
    }
}
