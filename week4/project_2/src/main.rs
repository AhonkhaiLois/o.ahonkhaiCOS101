use std::io;

fn main() {
    println!("Employee Incentive Calculator");

    // Step 1: Input experience
    let mut experience = String::new();
    println!("Is the employee experienced? (yes/no): ");
    io::stdin().read_line(&mut experience).expect("Failed to read input");
    let experience = experience.trim().to_lowercase(); // convert to lowercase for easy comparison

    // Step 2: Input age
    let mut age = String::new();
    println!("Enter the age of the employee: ");
    io::stdin().read_line(&mut age).expect("Failed to read input");
    let age: u32 = age.trim().parse().expect("Please enter a valid number");

    // Step 3: Determine the incentive based on given conditions
    if experience == "yes" {
        if age >= 40 {
            println!("The incentive of the employee is ₦1,560,000 per year.");
        } else if age >= 30 && age < 40 {
            println!("The incentive of the employee is ₦1,480,000 per year.");
        } else if age < 28 {
            println!("The incentive of the employee is ₦1,300,000 per month.");
        } else {
            println!("No specific incentive rule for this age group.");
        }
    } else {
        println!("The incentive of the inexperienced employee is ₦100,000.");
    }
}


