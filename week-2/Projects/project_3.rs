fn main() {
	// allocate variables
	let principal:f64 = 210000.0;
	let rate:f64 = 5.0;

	// calculate the depreciation and amount after 3 years
	let d:f64 = 1.0 - (r/100.0);
	let d = d*d*d;
	let Amount = p * d;
	println!("The value after 3 years is N{}", Amount); 
}