/*
-> String : the string type ,which is provided by rust standard library rather than coded into the core 
             language 
        - growable 
        - mutable 
        - owned 
        - utf-8 encoded string type  

- when we refer  to "strings" in rust , we mostly refer to the String 0r the StringSlice &str
- Both types String ans stringslice arae UTF-8 encoded

-> Slices : slices let you reference a contiguous sequence of elements in the collection rather than the whole collection.
        - A slice is kind of reference so it does not have ownership
*/

fn main() {
 
 // define a string 
 let mut s = String::from("kedar");

 //push something to the string 
 s.push_str("chinna");
 println!("{}" , s);

 // replace some thing from the string 
 s.replace_range(5..s.len(), "");
 println!("{}" , s);

 //Question - 1
 let  new_string = String::from("new String");
 let first_word = return_first_word(new_string);
 println!("{}" , first_word); 
//  println!("{}" , new_string); 


// correct code 
let str = String::from("string new ");
let first_word_ref = correct_return_first_word(&str);

println!("{}" , str);
println!("{}" , first_word_ref);


}


// question - 1
/*
Write a function that takes a string as an input and returns the first word of the string
*/

/*
-> Problems in the question-1 return_first_word
    - we are creating a new string which is stored in the heap
    - now we have tow strings in the heap "new String" ans "new" which takes extra memory in the heap
    - although new_string has now lost the ownership but still it takes the memory in the heap 
        so we should avoid this 
    - we want a bview of the first word not the copy of the firstword(storing the firstword in heap) 
*/

fn return_first_word(str :String) -> String{
    let mut new_str = String::from("");

    for i in str.chars(){
        if i == ' '{
            break;
        }
        // convert the char to string and then add to the new_str
        //to_string accepts &self as an arguments so we pass the reference of the char
        new_str.push_str(&i.to_string());
    }

    return new_str;
}

//correct function

fn correct_return_first_word(str : &String) -> &str{
    let mut index = 0;
    for (_ , i) in str.chars().enumerate(){
        if i == ' '{
            break ;
        }
        index = index + 1;
         
    }

    return &str[0..index];
}
 