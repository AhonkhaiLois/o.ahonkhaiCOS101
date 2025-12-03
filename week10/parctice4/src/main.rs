fn main() {
    // a list of nos
    let v = vec![15,25,35,45,55];

    //pass a reference (&v). Ownership is NOT moved.
    print_vector(v.clone()) ;

    //v is still valid because we only borrrwoed it
    //This line now compiles without error.
    println!("{}", v[0] );

}

//The function now accepts a reference (&Vec<i32>)
fn print_vector(x:Vec<i32>) {
    println!("Inside print_vector function {:?}", x );
}

