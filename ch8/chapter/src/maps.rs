use std::collections::HashMap;
fn creating() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
}
fn creating_collect() {
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();
}
fn ownership() {
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // field_name; // value used after move
}
fn accessing() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yello"), 50);
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
}
fn iterating() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yello"), 50);
    for (key, value) in &scores {
        println!("{}: {}", key, value)
    }
}
#[test]
fn overwriting() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    assert_eq!(format!("{:?}", scores), "{\"Blue\": 25}");
}
#[test]
fn entry() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    assert_eq!(scores["Blue"], 10);
    assert_eq!(scores["Yellow"], 50);
}
#[test]
fn updating() {
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    assert_eq!(map["world"], 2)
}
