use std::collections::HashMap;
use std::io;

// Helper function to read input from the terminal.
fn get_input() -> Option<String> {
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer).is_err() {
        println!("Error reading input. Please try again.");
    }
    
    let input = buffer.trim().to_owned();
    if input.is_empty() {
        None
    } else {
        Some(input)
    }
}

// STAGE 1: Add & View Bills


fn add_bill(bills: &mut HashMap<String, f64>) {
    println!("\nEnter bill name (or press Enter to go back):");
    let name = match get_input() {
        Some(name) => name,
        None => return, // "Go back if I change my mind"
    };

    println!("Enter bill amount (or press Enter to go back):");
    let amount_str = match get_input() {
        Some(amount) => amount,
        None => return,
    };

    // Parse the string input into a float (f64)
    let amount: f64 = match amount_str.parse() {
        Ok(amt) => amt,
        Err(_) => {
            println!("Invalid amount. Bill not added.");
            return;
        }
    };

    bills.insert(name, amount);
    println!("Bill added successfully!");
}

fn view_bills(bills: &HashMap<String, f64>) {
    if bills.is_empty() {
        println!("\nNo bills currently saved.");
        return;
    }
    
    println!("\n=== Your Bills ===");
    for (name, amount) in bills {
        println!("{}: {:.2}", name, amount);
    }
    println!("==================");
}

// STAGE 2: Remove Bills

fn remove_bill(bills: &mut HashMap<String, f64>) {
    view_bills(bills);
    if bills.is_empty() {
        return; // Nothing to remove
    }

    println!("\nEnter the name of the bill to remove (or press Enter to go back):");
    let name = match get_input() {
        Some(name) => name,
        None => return,
    };

    // The .remove() method returns Option<V>. 
    // If it finds the key, it removes it and returns Some(value).
    if bills.remove(&name).is_some() {
        println!("Bill removed.");
    } else {
        println!("Bill not found.");
    }
}

// STAGE 3: Edit Bills

fn edit_bill(bills: &mut HashMap<String, f64>) {
    view_bills(bills);
    if bills.is_empty() {
        return;
    }

    println!("\nEnter the name of the bill to edit (or press Enter to go back):");
    let name = match get_input() {
        Some(name) => name,
        None => return,
    };

    // Check if the bill actually exists before asking for a new amount
    if !bills.contains_key(&name) {
        println!("Bill not found.");
        return;
    }

    println!("Enter new amount (or press Enter to go back):");
    let amount_str = match get_input() {
        Some(amount) => amount,
        None => return,
    };

    let amount: f64 = match amount_str.parse() {
        Ok(amt) => amt,
        Err(_) => {
            println!("Invalid amount. Edit cancelled.");
            return;
        }
    };

    // Using .insert() on an existing key will overwrite the old value
    bills.insert(name, amount);
    println!("Bill updated!");
}

// MAIN PROGRAM LOOP

fn main() {
    // Initialize our HashMap. Keys are Strings (Bill Name), Values are f64 (Amount)
    let mut bills: HashMap<String, f64> = HashMap::new();

    loop {
        println!("\n--- My Interactive Bill Manager ---");
        println!("Welcome to Immaculate's Interactive Bill Manager! \n
        Have fun! Here are your options: \n");
        println!("1. Add a Bill");
        println!("2. View your Bills");
        println!("3. Remove your Bills");
        println!("4. Edit your Bills");
        println!("5. Exit the program");
        println!("Choose an option:");

        match get_input() {
            Some(choice) => match choice.as_str() {
                "1" => add_bill(&mut bills),
                "2" => view_bills(&bills),
                "3" => remove_bill(&mut bills),
                "4" => edit_bill(&mut bills),
                "5" => {
                    println!("Goodbye, till next time!");
                    break; // Breaks the loop and exits the program
                }
                _ => println!("Invalid choice, please select 1-5."),
            },
            None => println!("Please enter a selection."),
        }
    }
}