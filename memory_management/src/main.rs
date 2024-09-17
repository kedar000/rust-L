// main is added to the stack
fn main() {
    create_stirng();
    // create_string function added to the stack 
}

fn create_stirng(){

    // i32 block is added to the satck 
    // "hi this is an string " added to the heap 
    // an 132 block in the stack stores the address of the string present in the heap 
    let s = String::from("hi this is an new string");

    println!("{}" , s);

    // after leaching this step the i32 block present in the satck vanishes 
    // now the string present in the heap doest not have a block which stores there addres in the stack
    // so this also vanishies from the heap

// ** Basically every thing present int the heap has an Owner and there can be only one Owner
// ** if the Owner goes out of scope the value is also dropped from the heap

    // if it is  c++ then the string remains in the heap 

}