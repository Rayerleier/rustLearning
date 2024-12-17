use std::collections::HashMap;
use std::io;
fn main() {
    let mut company: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        println!("Please add employee(e.g., 'Add Sally to Engineering')");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim();
        if input.eq_ignore_ascii_case("exit") {
            break;
        }

        let words: Vec<&str> = input.split_whitespace().collect();
        if words.len() < 2 {
            println!("Invalid command. Please try again.");
            continue;
        }

        match words[0].to_lowercase().as_str() {
            "add" => {
                if words.len() < 4 || words[2].to_lowercase() != "to" {
                    println!("Invalid command format for adding employee. Use 'Add [Name] to [Department]'.");
                } else {
                    let name = words[1].to_string();
                    let department = words[3..].join(" ");
                    add_employee(&mut company, &name, &department);
                }
            }
            "list" => {
                if words.len() == 2 && words[1].to_lowercase() == "all" {
                    list_all_employees(&mut company);
                } else if words.len() == 2 {
                    let department = words[1..].join(" ");
                    list_employees_by_department(&company, &department);
                } else {
                    println!("Invalid command format for listing employees. Use 'List [Department]' or 'List all'.");
                }
            }
            _ => {
                println!("Unknown command. Please try again.");
            }
        }
    }
}

fn add_employee(company: &mut HashMap<String, Vec<String>>, name: &str, department: &str) {
    let employees = company.entry(department.to_string()).or_insert(Vec::new());
    employees.push(name.to_string());
    employees.sort();
    println!("Added {} to {}", name, department);
}

fn list_employees_by_department(company: &HashMap<String, Vec<String>>, department: &str) {
    match company.get(department) {
        Some(employees) => {
            println!("Employees in {}:", department);
            for employee in employees {
                println!("{}", employee);
            }
        }
        None => {
            println!("No employees found in {}", department);
        }
    }
}

fn list_all_employees(company: &mut HashMap<String, Vec<String>>) {
    for (department, employees) in company {
        println!("Employees in {}:", department);
        for employee in employees {
            println!("{}", employee);
        }
    }
}
