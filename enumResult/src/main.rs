// Result enum : used to throw an error 

// just like Option enum 
// enum Result{
//     OK(),     - same as Some in Option 
//     err
// }i 


// the code below allows us to read some other file lets say file A 
// if the file A exist then we return OK else if return Err and continue to the remaining code
// if we dont return an Err then the programm will crash there 

use std::fs::read_to_string;
fn main() {
  let result = read_to_string(( "a.text")); // this returns an Result  

  // if a.text does not exist then we return err 
  
  match result{
    Ok(data) => println!("{}" ,data),
    Err(err) => println!("Error while reading the file"),
  }

  // most of use return "" or -1 if there exist no file as a.text but what if the a.text file conatains value "" ot -1 
  // to avoid this we use Result enum
}
