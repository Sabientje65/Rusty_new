pub mod exc {
    pub trait Exercise {
        fn run(&self);
        
        // fn run_self<T: Self>(&self){
        //     T::run();
        // }
    }
}
