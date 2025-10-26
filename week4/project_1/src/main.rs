use std::io;  // for input/output

fn main() {
    println!("Quadratic Equation Solver");
    println!("Equation form: ax² + bx + c = 0");

    // Step 1: Input values for a, b, and c
    let mut a = String::new();
    let mut b = String::new();
    let mut c = String::new();

    println!("Enter value for a: ");
    io::stdin().read_line(&mut a).expect("Failed to read input");
    let a: f64 = a.trim().parse().expect("Please enter a valid number");

    println!("Enter value for b: ");
    io::stdin().read_line(&mut b).expect("Failed to read input");
    let b: f64 = b.trim().parse().expect("Please enter a valid number");

    println!("Enter value for c: ");
    io::stdin().read_line(&mut c).expect("Failed to read input");
    let c: f64 = c.trim().parse().expect("Please enter a valid number");

    // Step 2: Calculate the discriminant
    let discriminant = b * b - 4.0 * a * c;

    // Step 3: Determine the nature of the roots
    if discriminant > 0.0 {
        let root1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let root2 = (-b - discriminant.sqrt()) / (2.0 * a);
        println!("The equation has two distinct real roots:");
        println!("Root 1 = {:.2}", root1);
        println!("Root 2 = {:.2}", root2);
    } else if discriminant == 0.0 {
        let root = -b / (2.0 * a);
        println!("The equation has one real root:");
        println!("Root = {:.2}", root);
    } else {
        println!("The equation has no real roots.");
    }
}
