fn main (){
    let s1 = String::from("Hello"); 
    let len = calculate_length(&s1); //borrow s1 ownership
                                     //this is borrowing, the reference data cant be modify by default

    println!("The length of '{s1}' is {len}."); //s1 is still valid and len is return form function
}
fn calculate_length(s: &String) -> usize{ //&string is crucial without & rust will expect string no reference
    s.len() //expression return usize
}