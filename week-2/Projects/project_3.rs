fn main() {
	// allocate variables
	let principal:f64 = 210000.0;
	let rate:f64 = 5.0;
	let n:f64 = 3.0;
	
	// calculate the depreciation and amount after 3 years
	let Amount = p * (1.0 - (r/100.0)).powf(n);
	println!("The value after 3 years is N{}", Amount); 
}