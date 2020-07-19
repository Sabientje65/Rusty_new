pub mod root {
    struct Test {
        name: String
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

            println!("veccie: {}", &v[0].name)    
        }
        
        println!("{}", t.name)
    }
}
