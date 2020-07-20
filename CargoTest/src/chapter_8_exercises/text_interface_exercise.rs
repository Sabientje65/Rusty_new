use std::collections::HashMap;

struct AddToDepartmentParameters {
    employee: String,
    department: String
}

enum SortDirection {
    None,
    Ascending,
    Descending
}

enum Command {
    AddToDepartment(AddToDepartmentParameters),
    List(String),
    Quit,
    None
}

impl Command {
    fn create_list_command(mut words: core::str::Split<char>) -> Command {
        match words.next() {
            Some(department) => Command::List(department.to_string()),
            _ => Command::None
        }
    }
    
    fn create_add_command(mut words: core::str::Split<char>) -> Command {
        let employee = words.next();
        words.next();

        match words.next() {
            Some(department) => {
                Command::AddToDepartment(AddToDepartmentParameters {
                    employee: employee.unwrap().to_string(),
                    department: department.to_string()
                })
            },
            _ => Command::None
        }
    }
    
    fn parse(str: &String) -> Command {
        let mut words = str.split(' ');
        
        match words.next() {
            Some(signature) => {
                match signature.to_lowercase().as_str() {
                    //List DEPARTMENT
                    "list" => Command::create_list_command(words),
                    //Add EMPLOYEE to DEPARTMENT
                    "add" => Command::create_add_command(words),
                    "quit" => Command::Quit,
                    _ => Command::None
                }
            }
            _ => Command::None
        }
    }
}

pub fn run(){
    let mut departments: HashMap<String, Vec<String>> = HashMap::new();
    let stdin = std::io::stdin();
    
    loop {
        let mut buffer = String::new();
        stdin.read_line(&mut buffer);
        
        let command = Command::parse(&buffer);
        
        match command {
            Command::None => {
                println!("Failed to parse command: {}", buffer);
            },
            Command::Quit => {
                break;
            }
            Command::List(department) => {
                match departments.get(&department) {
                    Some(employees) => {
                        println!("Employees in: {}", department);
                        
                        for employee in employees {
                            println!("{}", employee)
                        }
                    },
                    None => println!("Department not found")
                }
            },
            Command::AddToDepartment(parameters) => {
                println!("Adding {} to department {}", parameters.employee, parameters.department);
                
                departments
                    .entry(parameters.department)
                    .or_insert(Vec::new())
                    .push(parameters.employee);
            }
        }
    }
}