fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String { // dangle returns a reference to a String
                         
 
    let s = String::from("hello"); // s is a new String

    s // we cant return a reference to the String, s
      // because reference is create inside the function it will become invalid after s goes out of scope
      // if we return s the ownership move out with no problem
} // Here, s goes out of scope and is dropped, so its memory goes away.
  