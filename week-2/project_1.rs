fn main() {
	let p:f64 = 520000000.00;
	let r:f64 = 10.00;
	let n:f64 = 5.0;

	// compound interest
	let a= p* (1.0 +(r/100.00)) *n ;
	println! ("Amount is {}", a) ;
	let ci = a-p ;
	println!("compound interest is {}", ci);

}