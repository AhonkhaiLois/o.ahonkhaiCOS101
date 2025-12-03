fn main() {
    // v owns the  object in the heap

    let v = vec![101,250, 330, 400];

    //v2 is a reference ( a non-owning pointer ) owned by v
    let v2 = &v;

    //v is still valid because ownership was not moved
    //v2 is valid as reference.
    println!("v: {:?}", v );

    println!("v2 (reference) : {:?}", v2);
}
