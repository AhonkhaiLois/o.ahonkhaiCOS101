fn main() {
    //Using Vec::new()
    let v :Vec<i64> = Vec ::new();

    //Printing the size of Vector
    println!("\nThe  length of vec::new() is : {}", v.len());

    //Using macro
    let v = vec!["Grace","Effiong","Basil","Kareem","Susan"];

    //printing the size pf vector
    println!("\nThe length of vec macro is :{}",v.len());
}
