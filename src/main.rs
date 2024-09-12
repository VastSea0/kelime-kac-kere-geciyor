use std::collections::HashMap;
use std::io;

fn main() {
    println!("Hello, world!");
    let mut most_frequent_word = "";
    let mut highest_count = 0;
    let mut my_wonderfull_string = Default::default();

    io::stdin()
        .read_line(&mut my_wonderfull_string)
        .expect("Error reading from stdin");

    let my_wonderfull_string = my_wonderfull_string.to_lowercase();


    let mut word_count = HashMap::new();
    for word in my_wonderfull_string.split_whitespace() {
        let counter = word_count.entry(word).or_insert(0);
        *counter += 1;
    }

    for (word, &count) in word_count.iter() {
        if count > highest_count {
            most_frequent_word = word;
            highest_count = count;
        }
    }
    println!("Girilen metin: {}", my_wonderfull_string.trim());
    println!("En sık geçen kelime: '{}', {} kez geçti.", most_frequent_word, highest_count);


}
