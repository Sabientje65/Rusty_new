mod c8_vec {
    use std::collections::HashMap;
    use std::fmt;
    use std::fmt::Formatter;

    pub enum Action {
        Mean(Vec<u32>),
        Median(Vec<u32>),
        Mode(Vec<u32>)
    }
    
    impl Action {
        fn get_name(&self) -> String {
            match self {
                Action::Mean(_) => String::from("mean"),
                Action::Median(_) => String::from("median"),
                Action::Mode(_) => String::from("mode"),
                _ => String::from("unknown")
            }
        }
    }
    
    impl fmt::Display for Action {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{:?}", self.get_name())
        }
    }
    
    pub fn handle(act: &Action) -> u32 {
        match act {
            Action::Mean(slc) => mean(slc),
            Action::Median(slc) => median(slc),
            Action::Mode(slc) => mode(slc)
        }
    }
    
    fn mean(slc: &Vec<u32>) -> u32{
        let mut total: u32 = 0;
        
        for v in slc.iter() {
            total += v;
        }
        
        total / (slc.len() as u32)
    }
    
    fn median(slc: &Vec<u32>) -> u32 {
        let mut slc_cp: Vec<u32> = slc.to_vec();
        let mut idx = 0;
        let mut prev = 0;
        
        let sz = slc.len();
        
        loop {
            match slc_cp.get(idx) {
                Some(v_ref) => {
                    let v = v_ref.to_owned();
                    if prev <= v {
                        //Move on to next element
                        idx += 1;
                        prev = v;
                        continue;
                    }

                    //Remove both prev/current
                    slc_cp.remove(idx);
                    slc_cp.remove(idx - 1);

                    //Swap positions of prev/current
                    slc_cp.insert(idx - 1, v);
                    slc_cp.insert(idx, prev);

                    //Restart (because tired)
                    idx = 0;
                    prev = 0;
                },
                
                //All elements processed
                None => break
            }
        }
        
        match slc_cp.get(sz / 2) {
            Some(v) => v.to_owned(),
            None => 0
        }
    }
    
    fn mode(slc: &Vec<u32>) -> u32 {
        let mut cnt: HashMap<u32, u32> = HashMap::with_capacity(slc.len());
        // let mut mx = 0;

        for v in slc {
            *cnt.entry(v.to_owned()).or_insert(0) += 1
            // *cnt_v += 1
        }

        let mut max = (0, 0);
        
        for (v, c) in cnt {
            match c {
                c if c > max.1 => max = (v, c),
                _ => {}
            }
            
            // prev_max.0 = v;
        }
        
        max.0
    }
}

pub fn run(){
    use self::c8_vec::{Action, handle};
    
    let actions = vec![
        Action::Mean(vec![3,9,1,2,6]),    
        Action::Median(vec![5,1,9,2,7]),    
        Action::Mode(vec![1,5,5,2,2,5,3,3,4,4,4,5,5]),    
    ];

    for action in actions {
        println!("Action: {}, resulted in value: {}", action, handle(&action));
    }
}