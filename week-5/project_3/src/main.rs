use std::io;

fn main() {
    // print the menu
    println!("\n            MENU
Poundo Yam/Edinkaiko Soup (P) - N3,200
Fried Rice & Chicken (F) - N3,000
Amala & Ewedu Soup (A) - N2,500
Eba & Egusi Soup (E) - N2,000
White Rice & Stew (W) - N2,500");

    // get the inputs
    let mut input1 = String::new();
    let mut input2 = String::new();
    
    println!("\nWhat food would you like?(P, F, A, E or W)");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let food = input1.trim();
    
    println!("\nHow many portions would you like?");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let amt:f64 = input2.trim().parse().expect("Not a valid integer");

    // set the conditions
    let mut charge:f64 = 0.0;

    if food == "p" || food == "P" {
        charge = 3200.0 * amt;
    }
    
    else if food == "f" || food == "F" {
        charge = 3000.0 * amt;
    }
    
    else if food == "a" || food == "A" {
        charge = 2500.0 * amt;
    }
    
    else if food == "e" || food == "E" {
        charge = 2000.0 * amt;
    }    
    
    else if food == "w" || food == "W" {
        charge = 2500.0 * amt;
    }
    
    if charge > 10000.0 {
        charge = charge * (95.0/100.0);
    }
    
    println!("\nYour total charge is N{}", charge);
}