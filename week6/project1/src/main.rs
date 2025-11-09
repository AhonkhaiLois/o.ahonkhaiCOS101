use std::io;

fn main() {
    // Step 1: Display the menu
    println!("Welcome to Rust Restaurant!");
    println!("Here is our menu:\n");
    println!("P = Poundo Yam/Edinkaiko Soup  - ₦3,200");
    println!("F = Fried Rice & Chicken        - ₦3,000");
    println!("A = Amala & Ewedu Soup          - ₦2,500");
    println!("E = Eba & Egusi Soup            - ₦2,000");
    println!("W = White Rice & Stew           - ₦2,500");
    println!("\nPlease enter the letter of your choice:");

    // Step 2: Take user input
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read input");

    // Step 3: Remove extra spaces or newline
    let choice = choice.trim().to_uppercase();

    // Step 4: Match the choice and display result
    match choice.as_str() {
        "P" => println!("You ordered Poundo Yam/Edinkaiko Soup - ₦3,200"),
        "F" => println!("You ordered Fried Rice & Chicken - ₦3,000"),
        "A" => println!("You ordered Amala & Ewedu Soup - ₦2,500"),
        "E" => println!("You ordered Eba & Egusi Soup - ₦2,000"),
        "W" => println!("You ordered White Rice & Stew - ₦2,500"),
        _ => println!("Sorry, that option is not on the menu."),
    }

    println!("Thank you for your order!");
}

