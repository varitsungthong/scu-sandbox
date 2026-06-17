fn main() {
    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn plus_one(y: i32) -> i32 {
    y + 1
}

//the variable use in fn plus_one dont have to match up with the variable use in other function