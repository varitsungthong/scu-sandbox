use std::collections::HashMap;
fn main() {
    let loot_drop = ["gold_coin", "exp","exp", "sheild" , "potion","potion","potion"];

    let mut inventory = HashMap::new();

    for Btring in loot_drop.iter()   {       
        let hahaha:&mut i32 = inventory.entry(Btring.to_string()).or_default();
        *hahaha +=1 ;
    } 
    for (key,value) in inventory.iter() {
        println!("Item: {key} | Count : {value}")
    }
}
