use core::num;
use std::ptr::null;

// we should use enum types any time you need to represent a fixed set of constants. 
enum Direction{
    North , 
    South ,
    East ,
    West
}

enum Shape{
    Rectangle(f64 , f64) , // accpets two arguments lenght and breasdth of flote 64 bits each
    Circle (f64), //accepts 1 arguments radis 
    Square (f64)

}



// this can only have 4 values which is easy and precise 
// we can also use a strinfs in the place of enums but then movearound function sjould have the string type which will eventually become difficult 

fn move_around(direction : Direction){
    println!("hi there ")
}

//calculate the area by using match for getting all the shapes area
// this is known as pattern matching  
fn calculate_area(shape : Shape) ->f64 {

     let area = match shape {
         Shape::Circle(r)=> 3.14 * r * r,
         Shape::Rectangle(a , b ) => a*b,
         Shape::Square(l)=> l*l,
     };

     return area;
}

// there are some inbuild ebums in rust 
// -> Options and Results
 // 1 . to have null values
 // 2 . error handiling

// Option enum is used to return none/null/nil values

// normal without using Option
fn find_first_a(s : String)-> i32{

    for(index , char ) in s.chars().enumerate(){
        if char == 'a'{
            return index as i32;
        }
    }

    return -1; // this is good but the index cannot be negative it is better to return a null in place of -1
}

// using an Option 
//  enum Option<T>{
//     None ,
//     Some(T)
//  }

fn find_first_a_using_Option(s : String) -> Option<i32>{
    for (index , char ) in s.chars().enumerate(){
        if char =='a' {
            return Some(index as i32);
        }
    }

    return None;
}


fn main() {
    let my_direction = Direction::North;
   //  println!("{}" , my_direction);
   move_around(my_direction);


   // for shapes
   let rect = Shape::Rectangle(1.0, 10.0);
   let circle = Shape::Circle(1.2);
   let square = Shape::Square(4.0);

   //enums which takes parameters
   println!("{}" , calculate_area(rect));
   println!("{}" , calculate_area(circle));
   println!("{}" , calculate_area(square));

   // for Option enum
   let s = String::from("sky");
   println!("{}" , find_first_a(s));

    let sky = String::from("sky");
   // using an Option 
   println!("{:?}" , find_first_a_using_Option(sky));
}