fn get_coordinate() -> (i32, i32) {
    (4, 7) 
}

fn main() {
    let (_x, y) = get_coordinate();

    if y > 5 {
        println!("greater than 5");
    } else if y < 5 {
        println!("less than 5");
    } else {
        println!("equal to 5");
    }
}