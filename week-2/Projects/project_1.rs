fn main() {
	// allocate variables
	let principal:f64 = 520_000_000.0;
	let rate:f64 = 10.0;
	let n:f64 = 5.0;

	//calculate the appreciation, amount and compound interest
	let ap:f64= 1.0 + (rate / 100.0);
	let amount = principal * ap.powf(n);
	let compound_interest = amount - principal;
	println!("The compound interest is N{}", compound_interest);

}