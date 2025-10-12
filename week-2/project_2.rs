fn main() {
	//our arrays
	let x:[f64;5] =[450000.00, 1500000.00, 750000.00, 2850000.00, 250000.00];//values
	let f:[f64;5] =[2.00, 1.00, 3.00, 3.00, 1.00];// frequencies
	//find totals with indexing
	let total_fx = f[0]*x[0]+f[1]*x[1]+f[2]*x[2]+f[3]*x[3]+f[4]*x[4];
	let total_f= f[0]+f[1]+f[2]+f[3]+f[4];
	println!("sum is {}",total_fx );
	// find mean
	let mean= total_fx / total_f;
	println!("Average is {}", mean);
}