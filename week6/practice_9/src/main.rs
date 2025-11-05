fn main() {

    let a:i32 = 10;
    let b:i32 = 20;

    println!("Value of A:{}",a);
    println!("Value of B : {}",b);

    let mut res = a>b ;
    println!("A greater than B: {}",res );

    res = a<b;
    print!("A lesser than B {}",res ); 

    res = a>= b;
    println!("A greater than pr equal to b: {}",res );

    res = a<=b;
    println!("A is lesser than or equal to B:{}",res );

    res = a==b ;
    println!("A is equal to B: {}",res );

    res = a!=b;

    println!("a is not equal to B:{}",res);
    
}
