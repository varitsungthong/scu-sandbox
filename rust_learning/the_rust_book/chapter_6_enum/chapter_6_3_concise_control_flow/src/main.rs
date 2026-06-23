#[derive(Debug)]
enum Usstate {
    Alabama, 
    Texas,
}




enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(Usstate),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(Usstate) =>  {println!("State quarter from {Usstate:?}!");
            25}
    }
}
fn main (){
    let coin = Coin::Quarter(Usstate::Texas);
    let mut count = 0;
    if let Coin::Quarter(Usstate) = coin {
        println!("State quarter from {Usstate:?}!");
    } else {
        count += 1;
    }
}
