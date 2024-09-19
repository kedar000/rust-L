//define a trate | example :just like interface in java 
pub trait Qualities {
    fn quality(&self) -> String; // we can have a default implementaion(write some code in funciton) insted of signature(return type)
}

// define a struct |  exmaple : just like creating a class in java
struct User{
    name : String,
    age : u32,
}

// User struct implements the Qualities trate just like class implementaion an interface in java
impl Qualities for User{
    fn quality(&self) -> String {
        return format!("user {} is {} year old" , self.name , self.age);
    }
}


// Trates as parameters 
/*
suppose we want to create a function called notify which takes the inputs and that inputs must implement a paricular trait 
*/
fn notify(u : &impl Qualities){
    // use takes only Qualities because we 100% know that any funciton that implement the Qualities must have the Quality funciton 
    println!("{}" , u.quality());
}
 
fn main() {

    let user = User{
        name : String::from("kedar"),
        age : 19,
    };
    println!("{}" , user.quality());
    



    // callin the notify funciton
    notify(&user);
    // notify(1);       these both gives the error b/c int ans String does not implement the Qualities trate
    // notify(String:;from("kedar"));
}


/*
-> behind the hood the notify function is converted like this 
pub fn notify<T : Qualities>(u : &T){
    println!("breaking news : {}" , u.quality())
}

- in we want to add multiple trait bounds 
pub fn notify<T : Qualities + new_trate + ....>(u : &T){
    println!("breaking news : {}" , u.quality())
}

*/
