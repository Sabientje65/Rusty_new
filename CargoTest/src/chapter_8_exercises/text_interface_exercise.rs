use std::collections::HashMap;

enum SortDirection {
    Ascending,
    Descending
}

enum Command {
    AddToDepartment(String, String),
    ListDepartment(String, Option<SortDirection>),
    ListCompany(Option<SortDirection>),
    Quit,
    None
}

impl Command {
    fn extract_sort_direction(mut words: core::str::Split<char>) -> Option<SortDirection> {
        //sorted ascending/descending
        
        loop {
            match words.next() {
                Some(potential_direction) => {
                    match potential_direction.to_lowercase().as_str() {
                        "sorted" => {
                            match words.next() {
                                Some(direction) => {
                                    match direction.to_lowercase().trim() {
                                        "ascending" => return Option::Some(SortDirection::Ascending),
                                        "descending" => return Option::Some(SortDirection::Descending),
                                        value => {
                                            println!("Direction {} unknown", value);
                                            // Option::None
                                        }
                                    }
                                },
                                _ => {
                                    println!("Direction not found");
                                    // Option::None
                                }
                            };
                        },
                        value => {
                            println!("Failed to match {} for sorting", value)
                        }
                    }
                },
                _ => break
            }
        }

        return Option::None;
    }
    
    fn create_list_command(mut words: core::str::Split<char>) -> Command {
        //List employees of Department
        //List all employees
        
        loop {
            match words.next() {
                Some(token) => {
                    match token.to_lowercase().as_str() {
                        "all" => {
                            //Assume next token is 'employees', consume next token
                            words.next();
                            
                            return Command::create_company_list_command(words)
                        },
                        "employees" => {
                            //Assume next token is 'of'
                            words.next();

                            return Command::create_department_list_command(words)
                        },
                        _ => continue
                    }
                },
                None => break
            }
        }

        Command::None
    }
    
    fn create_company_list_command(mut words: core::str::Split<char>) -> Command {
        Command::ListCompany(
            Command::extract_sort_direction(words)
        )
    }
    
    fn create_department_list_command(mut words: core::str::Split<char>) -> Command {
        match words.next() {
            Some(department) => {
                Command::ListDepartment(
                    department.to_owned(),
                    Command::extract_sort_direction(words)
                )
            },
            _ => Command::None
        }
    }
    
    fn create_add_command(mut words: core::str::Split<char>) -> Command {
        let employee = words.next();
        words.next();

        match words.next() {
            Some(department) => {
                Command::AddToDepartment(
                    employee.unwrap().to_string(),
                    department.to_owned()
                )
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

fn sort(items: &mut Vec<&String>, direction: &Option<SortDirection>) {
    match direction {
        Some(direction) => {
            match direction {
                SortDirection::Ascending => sort_ascending(items),
                SortDirection::Descending => sort_descending(items)
            }
        },
        None => {
            println!("Found no sort")
        }
    }
}

fn swap_by_index(items: &mut Vec<&String>, index_low: usize, index_high: usize) {
    let item_high = items.swap_remove(index_high);
    let item_low = items.swap_remove(index_low);
    
    items.insert(index_low, item_high);
    items.insert(index_high, item_low);
}

fn sort_descending(items: &mut Vec<&String>) {
    let mut i: usize = 1;
    
    loop {
        match items.get(i) {
            Some(current_item) => {
                match items.get( i - 1) {
                    Some(previous_item) => {
                        //Swap positions; restart
                        if previous_item.le(current_item) {
                            swap_by_index(items, i - 1, i);
                            i = 1;
                        }
                    },
                    None => continue
                }
            },
            None => break
        }
        
        i += 1;
    }
}

fn sort_ascending(items: &mut Vec<&String>) {
    println!("Sorting asc");
    
    let mut i: usize = 1;

    loop {
        match items.get(i) {
            Some(current_item) => {
                match items.get( i - 1) {
                    Some(previous_item) => {
                        //Swap positions; restart
                        if previous_item.ge(current_item) {
                            swap_by_index(items, i - 1, i);
                            i = 1;
                        }
                    },
                    None => continue
                }
            },
            None => break
        }

        i += 1;
    }
}

fn list_department(departments: &HashMap<String, Vec<String>>, department_name: &String, direction: &Option<SortDirection>){
    match departments.get(department_name) {
        Some(employees) => {
            //Copy employees into a new vector for ease
            let mut employees: Vec<&String> = employees.iter().collect();
            sort(&mut employees, direction);
            
            println!("Employees in: {}", department_name);

            for employee in employees {
                println!("{}", employee)
            }
        },
        None => println!("Department not found")
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
            },
            Command::ListCompany(direction) => {
                let mut department_names: Vec<&String> = departments.keys().collect();
                
                println!("Sorting departments");
                sort(&mut department_names, &direction);
                
                //department names are sorted now
                for department_name in department_names {
                    list_department(&departments, &department_name, &direction)
                }
            }
            Command::ListDepartment(department, direction) => {
                list_department(&departments, &department, &direction)
            },
            Command::AddToDepartment(employee, department) => {
                println!("Adding {} to department {}", employee, department);

                departments
                    .entry(department)
                    .or_insert(Vec::new())
                    .push(employee);
            }
        }
    }
}