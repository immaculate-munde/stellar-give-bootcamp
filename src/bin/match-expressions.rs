fn main() {
    // Matching on a Boolean
    
    let is_rust_fun = true; 
    match is_rust_fun {
        true => println!("it's true"),
        false => println!("it's false"),
    }

    // Matching on an Integer
   
    
    let my_number = 4; 
    match my_number {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("other"), 
    }
}