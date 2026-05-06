struct GroceryItem {
    quantity: i32,
    id_number: i32,
}

fn display_quantity(item: &GroceryItem) {
    println!("Quantity: {}", item.quantity);
}

fn display_id(item: &GroceryItem) {
    println!("ID Number: {}", item.id_number);
}

fn main() {
    let my_item = GroceryItem {
        quantity: 10,
        id_number: 1234,
    };

    display_quantity(&my_item);
    display_id(&my_item);
}