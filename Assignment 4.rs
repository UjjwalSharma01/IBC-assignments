struct Student {
    name: String,
    email: String,
    phone: String,
    id: u32,
}

fn main() {
    
    let mut students: Vec<Student> = Vec::new();

    
    students.push(Student {
        name: String::from("xyz"),
        email: String::from("xyz@example.com"),
        phone: String::from("123"),
        id: 1,
    });

    students.push(Student {
        name: String::from("abc"),
        email: String::from("abc@gmail.com"),
        phone: String::from("123"),
        id: 2,
    });

    students.push(Student {
        name: String::from("mno"),
        email: String::from("mno@gmail.com"),
        phone: String::from("123"),
        id: 3,
    });

    students.push(Student {
        name: String::from("pqr"),
        email: String::from("pqr@gmail.com"),
        phone: String::from("123"),
        id: 4,
    });

    students.push(Student {
        name: String::from("stu"),
        email: String::from("stu@gmail.com"),
        phone: String::from("123"),
        id: 5,
    });

    let index = 7; // Accessing an index that is out of bounds
    let student = students.get(index).expect("Student index out of bounds");
    
    println!("Student {} details:", student.id);
    println!("Name: {}", student.name);
    println!("Email: {}", student.email);
    println!("Phone: {}", student.phone);
}
