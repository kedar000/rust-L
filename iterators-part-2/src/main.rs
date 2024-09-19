/*
Types of Iterators : 
-> consuming Adaptors : Methods that call next are called consuming adaptors , because calling them uses up the iterator
-> Iterator Adaptor : Iterator adapteors defined on the iterator trait that dont consume the 
                      iterator.insted , they produce different different iterators by cahnging some aspects of the original iterator
*/
fn main() {
    let vec1 = vec![1, 2, 3, 4, 5, 6];
    
    //consuming adapters 
    // giving the reference of the vec1 to iter_1
    let iter_1 = vec1.iter();

    //using the iter as the sum and the iter_1 is consumed 
    let sum : i32 = iter_1.sum();

    println!("{}" , sum);
    // println!("{}" , iter_1);

    // for i in iter_1{
    //     println!("{}" , i);
    // }
    // we cant implement the above for loop because the iter_1 val is used up in the sum 
    
    println!("{:?}" , vec1);


    //----------------------------------------------------------------

    // map and filter are the exaples of the interator adaptors

    println!("Interator Adaptor :");

    let vec_2 = vec![10 , 9 , 8 , 7];
    let iter_v2 = vec_2.iter();

    // this returna an itreators to the iter2_v2
    let iter2_v2 = iter_v2.map(|x| x+1);

    // we can iterate through the iter2-v2
    for i in iter2_v2{
        println!("{}" , i);
    }

    // before changing the values in the map the values of vec-2
    println!("{:?}" , vec_2);

    // filter 

    let a = vec_2.iter();
    let iter_filter = a.filter(|x| *x%2 == 0);
    for i in iter_filter{
        println!("{}" , i);
    }


    //if we need to convert an iterator into a vector then we use collect() to do this  

}
