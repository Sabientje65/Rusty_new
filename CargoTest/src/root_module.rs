pub mod root {
    // use std::fmt::Pointer;

    use std::alloc::Layout;

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
