use std::io;

fn find_shortest_word(sentence: &str) -> Option<&str> {
    let words: Vec<&str> = sentence.split_whitespace().collect();
    if let Some(shortest) = words.iter().min_by_key(|&word| word.len()) {
        Some(shortest)
    } else {
        None
    }
}

fn main() {
    println!("Enter a string of words:");
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let sentence = input.trim();

    if let Some(shortest_word) = find_shortest_word(sentence) {
        println!("The shortest word in the sentence is: {}", shortest_word);
    } else {
        println!("No words found in the input.");
    }
}
