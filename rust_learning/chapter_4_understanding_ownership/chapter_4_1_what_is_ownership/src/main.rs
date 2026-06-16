fn main() {
    let s1 = String::from("Hello");
    let s2 = s1 ; // immediately invalidate s1 to prevent freeing the same memory 
                  // rust move string form s1 to s2 instead of copying it.
    println!("{s2}")
} //s2 goes out of scope rust return heap memory of s2
