fn main(){
	let p:f64= 510000.00;
	let r:f64= 5.00;
	let n:f64= 3.00;
	// depreciation
	let a = p* (1.0 +(r/100.00)).powf(n);
	println! ("amount is {}", a) ;
	let d= p-a ;
	println!("depreciation is {}", d);
}

