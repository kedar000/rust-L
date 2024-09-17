struct Rectangle{
    height : i32,
    width : i32,
}


// struct are more like classes in jva rather than object in js
// structs can implement function and manymore like classes

impl Rectangle {  // implementing an area function in Rectangle
    fn area(&self) -> i32{ 
        return self.height * self.width;
    }
}

impl Rectangle{
    // self is like current struct same like "this" in the class
    fn perimeter(&self) -> i32{
        return 2 * (self.height + self.width);
    }

    // what if we dont use the &self 
    // example
    fn debug()-> i32{
        return 1;
    }

}

fn main() {
    
    let rect = Rectangle{
        height : 10,
        width : 10,
    };

    println!("{}" , rect.area());
    println!("{}" , rect.perimeter());


    // print!("{}" , rect.debug());
    // we cannot call the debug using the object like rect

    // now we can treat the debug function as a static function which we call using the main class 
    println!("{}" , Rectangle::debug());
    // we called the debig function direct using the recatngle struct
}
