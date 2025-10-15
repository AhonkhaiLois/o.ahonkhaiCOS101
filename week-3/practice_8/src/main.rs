fn main() {
    let mut fees = 25_000; // 'mut' allows variable to be changed
    println!("fees is {}", fees);

    fees = 35_000; // now allowed
    println!("fees changed is {}", fees);
}