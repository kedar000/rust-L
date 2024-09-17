fn main() {

    // println! is a macro 
    // "{}" this is dynamically printing value 
    // whole statement is print the value that is returend by is_even in the place of "{}"
    println!("{}" , is_even(30));
}


fn is_even(num : i32) -> bool {
    if num % 2 == 0{
        return true 
    }
    return false
}
//i32 : signed value of 32 bits Range -2^32 <= num <= 2^32
// returns a boolean 
// if conditional statement
