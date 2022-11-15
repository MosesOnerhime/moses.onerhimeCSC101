use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();


    println!("Enter your age: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let age:f64 = input1.trim().parse().expect("Not a valid age");

    println!("Are you experienced?(y or n)");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let exp = input2.trim();

    if exp == "y" && age >= 40.0 {
        let inc:f64 = 1560000.0;
        println!("Your incentive is N{}", inc);
    }

    else if exp == "y" && 30.0 <= age && age < 40.0 {
        let inc:f64 = 1480000.0;
        println!("Your incentive is N{}", inc);
    }

    else if exp == "y" && 0.0 <= age && age < 28.0 {
        let inc:f64 = 1300000.0;
        println!("Your incentive is N{}", inc);
    }

    else if exp == "n" {
        let inc:f64 = 100000.0;
        println!("Your incentive is N{}", inc);
    }

}