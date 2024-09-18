// vectors :
// -  vectors allow you to store more than one value in an single data structure that puts
//   all the values next to each other in the memory 
// - this are dynamic in nature so theyare stored in the heap of the memory
// - just as a string
fn main() {

 // creating a vectore
  let mut vec = Vec::new();

  // inserting the values 
  vec.push(1);
  vec.push(2);
  vec.push(3);

  println!("{:?}" , vec);
  // {:?}  implementing the debug trate we will study this in trates 

  //-----------------------------------------------------------------
 
  // assignement - 1
  /*
  write a function that takes a vector as an input and returns a vector with even values
   */
  //creating a new vector 
  let mut vector1 = Vec::new();
  // without changing the ownership of vector1
  even_vector(&mut vector1);
  println!("{:?}" , vector1);

 //-----------------------------------------------------------------

  // Initialising the vector using the macros
  let new_vec = vec![1 , 2 , 4]; 
  println!("{:?}" , new_vec);


}

//assignment - 1
f n even_vector(vec : &mut Vec<i32>){
    for i in 1..100{
        if i % 2 == 0{
            vec.push(i);
        }
    }
}
