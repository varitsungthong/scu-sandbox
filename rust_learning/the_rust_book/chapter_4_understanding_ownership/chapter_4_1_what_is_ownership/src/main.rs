fn main () {
    let mut s = String::from("hello");
    s = String::from("ahoy");// assigning a new value to an existing variable cause rust to drop prior value

    println!("{s}, world!");
}