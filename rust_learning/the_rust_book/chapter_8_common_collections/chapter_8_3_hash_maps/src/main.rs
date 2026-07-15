
fn main() {

    use std::collections::HashMap;
let mut scores = HashMap::new();
let team_name = String::from("Arsenal");

scores.insert(team_name, 1);

println!("Added 1 goal for team: {}", team_name);
}