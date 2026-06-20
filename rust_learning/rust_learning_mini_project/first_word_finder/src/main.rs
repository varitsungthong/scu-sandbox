use std::io;
fn main() {
    let mut user_input_sentence = String::new();

    io::stdin().read_line(&mut user_input_sentence).expect("failed to read line");

    println!("The user input sentence is {user_input_sentence}");

    // {
    //     let first_word_length = first_word_length_function(&user_input_sentence);
    //     println!("The first word length is {first_word_length}") 
    // }
    let trimmed = user_input_sentence.trim();
    let first_word = first_word(&trimmed);
    println!("The first word is {first_word}");

    let first_word_length = first_word.len();
    println!("The first word length is {first_word_length}");

}

fn first_word(s:&str) -> &str{
    let bytes = s.as_bytes();
    for (i,item) in bytes.iter().enumerate(){
        if *item == b' ' {
            return &s[0..i]
        }
        
    }
    &s[..]
}

// fn first_word_length_function(w:&str) -> usize {
//     let bytes = w.as_bytes();
//     for (i,item) in bytes.iter().enumerate(){
//         if item == &b' ' {
//             let w = &w[0..i];
//             return w.len()
//         }
//     }
//     w.len()
// }