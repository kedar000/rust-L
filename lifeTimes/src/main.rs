fn main() {

     let longest;
    let a = String::from("kedar");
    {
        let b = String::from("kedar chinna");
        
        longest = longest_string(a, b);
    }

    // here longest still 
    println!("{}" , longest);
    // println!("{}" , longest_string(a, b));
    
}

// noraml function without lifetimes
fn longest_string(a : String , b : String) -> String{
    if a.len() > b.len(){
        a
    }else{
        b
    }
}


/*  fn longest_using_str(a : &str , b : &str) -> &str{
        if a.len() > b.len(){
            a 
        }else{
            b
        }
    }
*/
// this code gives us the error of the lifetime
// if "a" is longest and "b" is small then this code is fine
// if "b" is longest than "a" then this code will give a error of lifetime problem 

/*
we need to have the lifetime specification of a particular variable 
exapmle : A and B has both different lifetimes now we are returning the longest value so we need to specify the lifetime of the 
          returing value based on the inputs A and B lifetimes 

In the above example lifetime of A is 5 - 15 lines ( 10 lines ) ans lifetime of B is 7 - 8 lines (3 lines)
so the returning funciton will have life span of three lines
*/
