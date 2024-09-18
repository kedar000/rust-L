use std::vec;

// Iterators :
/*
- Iterator patterns allows us to perform some tasks on an sequence of items in turn 
- An iterator is responsible for the logiv iterating over item and determining when the sequence has finished 
- when we use iterators , we dont have to reimplement that logiv 
- in rust iterators are lazy , meaning they have no effect untile we call methods that sondumes the
  iterator to use it up
*/
fn main() {
  let v1 = vec![1 , 2, 3, 4];

  // immutable iteraters we cant modify the vectors elemets we can along read the vectir

  // this just borrows the vaues in the v1 . iter does not becnome the owner of v1
  let iter = v1.iter();

  for i in iter{
    println!("{}" , i);
  } 

  // to conform the ownership
  println!("{:?}" , v1);


  // if we want the iteratot to take  be mutable vectors
  let mut v2 = vec![1 , 2, 3, 4];

  // passing through the mutable iterator
  let v1_mut_iter = v2.iter_mut();

  // cahnging the values of the v2
  for i in v1_mut_iter{
    *i = 0; // mutable i32 refference 
  }

 // to conform the ownership of the v2 we print the v2 
  println!("{:?}" , v2);


  // what actually ghappens under the iterators which are hidden in the for loop
  let nums = vec![1, 0,2, 3, 4];
  let mut iter = nums.iter();

  // to perform this iter variable must be a mut because if we see the next() it expects the mut self(&mut self)  varuable as an argument
  while let Some(val) = iter.next(){
    println!("{}" , val);
  }


  

}
