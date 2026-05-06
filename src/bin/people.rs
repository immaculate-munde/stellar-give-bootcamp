struct Person {
    age: i32,
    name: String,
    favorite_color: String,
}

fn print_info(name: &str, color: &str) {
    println!("Name: {}, Favorite Color: {}", name, color);
}

fn main() {
    let people = vec![
        Person {
            age: 8,
            name: String::from("Immaculate"),
            favorite_color: String::from("Blue"),
        },
        Person {
            age: 25,
            name: String::from("Munde"),
            favorite_color: String::from("Green"),
        },
        Person {
            age: 10,
            name: String::from("Eugene"),
            favorite_color: String::from("Red"),
        },
        Person {
            age: 4,
            name: String::from("Innocent"),
            favorite_color: String::from("Yellow"),
        },
    ];

    for person in &people {
        if person.age <= 10 {
            print_info(&person.name, &person.favorite_color);
        }
    }
}