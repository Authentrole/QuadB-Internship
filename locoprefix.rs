use std::io;

fn longest_common_prefix(strings: &[String]) -> String {
    if strings.is_empty() {
        return String::new();
    }

    let first_string = &strings[0];

    for (i, &ch) in first_string.as_bytes().iter().enumerate() {
        for string in strings.iter().skip(1) {
            if i >= string.len() || string.as_bytes()[i] != ch {
                return first_string[..i].to_string();
            }
        }
    }

    first_string.to_string()
}

fn main() {
    println!("Enter a set of strings (separated by spaces):");
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let strings: Vec<String> = input.trim().split_whitespace()
        .map(|s| s.to_string())
        .collect();

    let common_prefix = longest_common_prefix(&strings);
    if common_prefix.is_empty() {
        println!("No common prefix found.");
    } else {
        println!("Longest common prefix: {}", common_prefix);
    }
}
