fn main() {
	// allocate variables
	let T:f64 = 2.0 * 450000.0;
	let M:f64 = 1.0 * 1500000.0;
	let H:f64 = 3.0 * 750000.0;
	let D:f64 = 3.0 * 2850000.0;
	let A:f64 = 1.0 * 250000.0;
	let n:f64 = 2.0+1.0+(3.0*2.0)+1.0;

	// calculate sum and average
	let sum = T + M + H + D + A;
	let Average = sum/n;
	println!("The sum of the sales record is {} and the average is {}.", sum, Average);

}