use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    // key and value
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    println!("value of team blue is {}", score);

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are INVALID at this point, 
    // try using them and see what compiler error you get!

    println!("value of team blue is {}", score);
}
