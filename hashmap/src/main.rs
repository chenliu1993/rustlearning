use std::collections::HashMap;

fn main() {
    // let mut scores = HashMap::new();

    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Yello"), 50);

    // let teams = vec![String::from("Blue"), String::from("Yellow")];
    // let init_scores = vec![10,50];
    // let score: HashMap<_, _> =
    //     teams.iter().zip(init_scores.iter()).collect();

    let text = String::from("Hello Hello Wonderful world");
    let mut dict = HashMap::new();

    for word in text.split_whitespace() {
        let count = dict.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", dict);
}
