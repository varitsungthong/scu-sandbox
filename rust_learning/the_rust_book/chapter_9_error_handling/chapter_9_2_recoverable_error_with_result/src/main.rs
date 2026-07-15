use std::fs::File;

fn main() {
    //open file return Result<T,E>
    let greeting_file_result = File::open("hello.txt");
    
    //use match to handle result variant that return
    let greeting_file = match greeting_file_result {
        Ok(file) => file, //if return ok this will return file inside ok value
        Err(error) => panic!("Problem opening the file: {error:?}"), //if return error this will call panic! macro
    };
}