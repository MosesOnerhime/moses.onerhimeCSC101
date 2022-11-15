use std::io;

fn main() {
    // input the values of a, b, and c
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
    
    println!("Enter the value of a:");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a:f64 = input1.trim().parse().expect("Not a valid number");
    
    println!("Enter the value of b:");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b:f64 = input2.trim().parse().expect("Not a valid number");
    
    println!("Enter the value of c:");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let c:f64 = input3.trim().parse().expect("Not a valid number");
    
    // find the descriminant and roots
    let d:f64 = b.powf(2.0) - (4.0 * a * c);
    let x1:f64 = ((-1.0 * b) + d.powf(0.5)) / (2.0 * a);
    let x2:f64 = ((-1.0 * b) - d.powf(0.5)) / (2.0 * a);

    
    // determine if the descriminant is positive or negative
    if d > 0.0 {
        println!("\nThere are two distinct roots.");
    }
    else if d == 0.0 {
        println!("\nThere is exactly one real root.");
        
    }
    
    else if d < 0.0 {
        println!("\nThere are no real roots.");
    }
    
    println!("\nThe roots are {} and {}", x1, x2);
    
    
}