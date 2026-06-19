    
fn main (){
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
         println!("{r1}")
    } // r1 goes out of scope here, so we can make a new reference with no problems.
    
   

    let r3 = &s;
    let r4 = &s;

    println!("{r3},{r4}")
    // Multiple immutable references are allowed because
    // none of them can modify the data.
    //if two or more pointer access data at the same time without synchronizer it may cause data race 

}