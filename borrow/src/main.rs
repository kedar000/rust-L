
//borrow 
// we just pass the refference to another variable 

// Rules :
// - at any given time , you can have either one mutable reference or any number of immutable reference 
// - refernce must always be valid 
fn main() {
    let s1 = String::from("kedar");

    // we only pass the refference of the string1 but the ownership is still with the s1
     print_string(&s1);

    println!("{}" , s1);


    // if we need the modify the original string we pass the refference of mut string
    let mut new_string = String::from("new String");
    // /passing the &mut as input which changes the original values 
    to_change_value(&mut new_string);
    println!("{}" , new_string)

}

fn print_string(s : &String){ // we accepts the string refernce as an input
    println!("{}" , s);
}

fn to_change_value(s2 : &mut String){
    s2.push_str("string");
}
