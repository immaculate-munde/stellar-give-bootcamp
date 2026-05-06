struct Student {
    name: String,
    locker_assignment: Option<i32>,
}

fn main() {
    let student1 = Student {
        name: String::from("Immaculate"),
        locker_assignment: Some(104),
    };

    let student2 = Student {
        name: String::from("Eugene"),
        locker_assignment: None,
    };

    println!("\nStudent: {}", student1.name);
    match student1.locker_assignment {
        Some(number) => println!("Locker Assignment: {}", number),
        None => println!("Locker Assignment: None"),
    }

    println!("---");

    println!("\nStudent: {}", student2.name);
    match student2.locker_assignment {
        Some(number) => println!("Locker Assignment: {}", number),
        None => println!("Locker Assignment: None"),
    }
}