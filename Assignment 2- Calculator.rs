use std::io;

fn add(a: &f64, b: &f64) -> f64 {
    a + b
}

fn subtract(a: &f64, b: &f64) -> f64 {
    a - b
}

fn multiply(a: &f64, b: &f64) -> f64 {
    a * b
}

fn divide(a: &f64, b: &f64) -> f64 {
    if *b != 0.0 {
        a / b
    } else {
        panic!("Cannot divide by zero");
    }
}

fn main() {
    println!("Calculator Operations: \n1. Addition\n2. Subtraction\n3. Multiplication\n4. Division");
    
    let mut choice = String::new();
    println!("Enter your choice (1/2/3/4):");
    io::stdin().read_line(&mut choice).expect("Failed to read input");
    let choice: u32 = choice.trim().parse().expect("Invalid input");

    println!("Enter first number:");
    let mut num1 = String::new();
    io::stdin().read_line(&mut num1).expect("Failed to read input");
    let num1: f64 = num1.trim().parse().expect("Invalid input");

    println!("Enter second number:");
    let mut num2 = String::new();
    io::stdin().read_line(&mut num2).expect("Failed to read input");
    let num2: f64 = num2.trim().parse().expect("Invalid input");

    let result = match choice {
        1 => add(&num1, &num2),
        2 => subtract(&num1, &num2),
        3 => multiply(&num1, &num2),
        4 => divide(&num1, &num2),
        _ => {
            println!("Invalid choice");
            return;
        }
    };

    println!("Result: {}", result);
}
