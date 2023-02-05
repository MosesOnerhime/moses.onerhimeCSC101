use std::io::Read;
use std::io;
use std::process::abort;

fn admin(){
    // Ask for password
    let mut input1 = String::new();
    println!("Input the password:          #Hint 'csc101' ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let choice = input1.trim();

    if choice == "csc101" {
        let mut file = std::fs::File::open("globacom_dbase.sql").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        print!("{}", contents);    
    }

    else {
        abort();
    }


}

fn project_man(){
    let mut file = std::fs::File::open("project_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);

}

fn employee(){
    let mut file = std::fs::File::open("staff_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);

}

fn customer(){
    let mut file = std::fs::File::open("customer_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);

}

fn vendor(){
    let mut file = std::fs::File::open("dataplan_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);

}

fn main(){
    // Check for department
    let mut input1 = String::new();
    println!("Please select your department(Choose '1' if you are an Administrator, '2' for Project manager, 
    \n'3' for Employee, '4' for Customer, '5' for Vendor:");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let choice = input1.trim();
    // Choose adequate function based on the input
    if choice == "1" {
        admin();
    }

    else if choice == "2" {
        project_man();
    }

    else if choice == "3" {
        employee();
    }

    else if choice == "4" {
        customer();
    }

    else if choice == "5" {
        vendor();
    }

    loop {
        let mut input1 = String::new();
        println!("\nWould you like to run the code again(y or n)?");
        io::stdin().read_line(&mut input1).expect("Not a valid string");
        let choice = input1.trim();

        if choice == "y" {
            main();
        }
        else if choice == "n" {
            break;
        }
    }
}