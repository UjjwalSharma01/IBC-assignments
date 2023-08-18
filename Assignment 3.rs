// USING VECTORS - > there is also an alternate solution to this problem using just arrays in a more simple manner, i wanted to make ut interactive so i used vectors

use std::io;

#[derive(Debug)]
struct Employee {
    employee_id: u32,
    employee_name: String,
    email: String,
    age: u32,
    phone_number: String,
}

impl Employee {
    fn new() -> Employee {
        println!("Enter employee details:");
        let mut input = String::new();

        println!("Employee Name:");
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let employee_name = input.trim().to_string();
        input.clear();

        println!("Employee ID:");
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let employee_id: u32 = input.trim().parse().expect("Invalid input");
        input.clear();

        println!("Email:");
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let email = input.trim().to_string();
        input.clear();

        println!("Age:");
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let age: u32 = input.trim().parse().expect("Invalid input");
        input.clear();

        println!("Phone Number:");
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let phone_number = input.trim().to_string();

        Employee {
            employee_id,
            employee_name,
            email,
            age,
            phone_number,
        }
    }
}

fn find_employee_by_id(employees: &[Employee], target_id: u32) -> Option<&Employee> {
    employees.iter().find(|&employee| employee.employee_id == target_id)
}

fn find_employees_by_age(employees: &[Employee], target_age: u32) -> Vec<&Employee> {
    employees.iter().filter(|&employee| employee.age == target_age).collect()
}

fn main() {
    let mut employees: Vec<Employee> = Vec::new();

    loop {
        println!("1. Add Employee");
        println!("2. Find Employee by ID");
        println!("3. Find Employees by Age");
        println!("4. Exit");
        println!("Choose an option:");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let choice: u32 = input.trim().parse().expect("Invalid input");

        match choice {
            1 => {
                let employee = Employee::new();
                employees.push(employee);
            }
            2 => {
                println!("Enter employee ID:");
                input.clear();
                io::stdin().read_line(&mut input).expect("Failed to read input");
                let target_id: u32 = input.trim().parse().expect("Invalid input");
                if let Some(employee) = find_employee_by_id(&employees, target_id) {
                    println!("Employee found: {:?}", employee);
                } else {
                    println!("Employee not found with ID: {}", target_id);
                }
            }
            3 => {
                println!("Enter employee age:");
                input.clear();
                io::stdin().read_line(&mut input).expect("Failed to read input");
                let target_age: u32 = input.trim().parse().expect("Invalid input");
                let matching_employees = find_employees_by_age(&employees, target_age);
                if matching_employees.is_empty() {
                    println!("No employees found with age: {}", target_age);
                } else {
                    println!("Employees with age {}: {:?}", target_age, matching_employees);
                }
            }
            4 => {
                println!("Exiting program.");
                break;
            }
            _ => println!("Invalid choice. Please select a valid option."),
        }
    }
}
