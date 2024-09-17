use chrono::Local;
use chrono::Utc;

// adding packages 
// we need add the package chrono to use LocalTime and UtcTime
// we can do that by command cargo add chrono
fn main() {
    let now = Local::now();
    //Loacl is a struct and now() does not take &self as an argument so we can directly call the now() on Struct Local
    
    let global = Utc::now(); 
    println!("current time india is {}" , now);
    println!("current global time is {}" , global);
}
