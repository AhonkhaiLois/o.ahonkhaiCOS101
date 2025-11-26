use std::fs;

fn main() {
    fs::remove_file("New Text Document.txt").expect("could not remove file");
    println!("Hello, world!");

}
