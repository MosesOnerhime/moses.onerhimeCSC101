fn main() {
    // Using Vec::new()
    let v : Vec<i64> = Vec::new();

    // printing the size of the vector

    println!("\nThe lenght of the vector is: {}",v.len() );

    // Using a macro 
    let v = vec!["Grace", "Effiong", "Basil", "Kareem", "Susan"];

    println!("\nThis is a vector:{:?} ",v);
    // printin a vector size
    println!("\nThe length of the vec marco is:{} ",v.len());
}
