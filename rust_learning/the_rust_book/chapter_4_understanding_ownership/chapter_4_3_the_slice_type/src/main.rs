//Write a function that takes a string of words separated by spaces and 
//returns the first word it finds in that string. 
//If the function doesn’t find a space in the string, 
//the whole string must be one word, so the entire string should be returned.
// This approach does NOT use slices and is intentionally shown as a less ideal design and fail to return the first word.
fn main(){
    let s1 = String::from("Hello meow maow.");
    let word = first_word(&s1);

    println!("the number of char form first word is {word}.");
    //getting the string value reference form the first word of sentence is very tedious and error prone without the slice type.
}
fn first_word(s : &String) -> usize {
   let bytes = s.as_bytes(); //convert string to array of bytes
   for(i, &item) in bytes.iter().enumerate(){
    //iter return each element in collection method
    //enumerate wrap the result of iter and return each element as part of a tuple instead.
    //the first element of tuple returned frome enumerate is index.
    //the second element of tuple is a reference to the element. 
    if item == b' '{
        return i;
        }
    }
    s.len()

   }
