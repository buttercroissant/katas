use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("Blue team score: {score}");

    let unknown_team_score = scores.get("unknowncolor").copied().unwrap_or(0); // 0 fallback value for unknown key
    println!("Unknown team score: {unknown_team_score}");

    for (key, value) in &scores {
        println!("{key}: {value}");
    }
}
