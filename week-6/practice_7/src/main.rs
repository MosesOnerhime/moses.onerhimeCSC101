fn main() {
  // Arrays with data type (explict integer data type)
  let arrl:[i32;4] =[10, 20, 30, 40];
  println!("\nArray with data type");
  println!("array is {:?}", arrl);
  println!("array size is {}",arrl.len());

  //Array withou data type
  let arr2 =[10, 4, 20, 7, 45, 51, 444, 30, 40];
  println!("\nArray without data type");
  println!("array is {:?}", arr2);
  println!("array size is {}",arr2.len());

  // Array with default value that crates and 
  // initializes all its elements with a default value of -1.
  let arr3:[i32;8] =[-1;8];
  println!("\nArray with default value");
  println!("array is {:?}", arr3);
  println!("array size is {}",arr3.len());


}
