fn main() {
   //Name vector
   let name  = vec!["Mary","Tishe","Ife","Ade","Mark","Sally","Greg"];

   // Age vector

   let age = vec![16,17,12,13,14,15,17];

   println!("Age allocation:\n");
    //loop to iternate elements in vector
    for i in 0..age.len(){
        println!("{} is {} years old\n",name[i],age[i] );
    }
 
}
