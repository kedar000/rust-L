 /*
HashMap : hashmaps stores a key values pair in rust . similar to hashmaps in java
import ->  use std::collections::HashMap
*/

use std::collections::HashMap;
fn main() {
 let mut map = HashMap::new();
 map.insert( String::from("kedar") , 1);
 map.insert( String::from("tarun") , 2);
 map.insert( String::from("somu") , 3);

 println!("{:?}" , map);

 // this returns an Options because what if i enter "akjsbda" this is not in the hashmap
 // so we should return null so it returns an null 
 let first_user_name = map.get("kedar"); // try to cahnge the name and run again  

 //matching the enum 
 match first_user_name {
     Some(user_no) => println!("user number {}" , user_no),
     None => println!("user not found in the hashmap")
 }

 //-------------------------------------------------------------------------------------
 /*
  Assignment - 1 :
write a function that takes a vector of tuples and returns a hashmpa where the keys are the unique keys form the 
input tuples and the values are vectos of all corresponding values associated with each key
 */

 // this is a vector with type tuple
 let  tup_vec = vec![(String::from("kedar") , 1) , (String::from("tarun") , 2) , (String::from("somu") , 3)];
 
 //vector converted to hashmap with out changing the ownership of vector
 let new_map = group_all_data(&tup_vec);
 println!("{:?}" , new_map);

 // original vectore of type tuple
 println!("{:?}" , tup_vec);

 //-------------------------------------------------------------------------------------
}


// Assignment ans 
fn group_all_data(vec : &Vec<(String, i32)>) -> HashMap<&String , &i32>{
    let mut new_map = HashMap::new();
    for (str , int) in vec{
        new_map.insert(str, int);
    }

    return new_map;
}
