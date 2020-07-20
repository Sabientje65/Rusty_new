fn to_pig_latin(str: &String) -> String {
    let mut chars: Vec<char> = str.chars().collect();
    let first_char = chars.remove(0);
    
    let consonants: Vec<char> = vec!('a', 'e', 'u', 'i', 'o');
    
    chars.push('-');
    
    match first_char {
        v if !consonants.contains(&v)=> {
            chars.push(v);
        },
        _ => chars.push('h')
    };
    
    chars.push('a');
    chars.push('y');
    
    let mut s = String::with_capacity(chars.len());

    for char in chars {
        s.push(char)
    }
    
    s
}

pub fn run(){
    let test_cases = vec![
        String::from("first"),
        String::from("apple")
    ];

    for test_case in test_cases {
        println!("{} -> {}", test_case, to_pig_latin(&test_case))
    }
}