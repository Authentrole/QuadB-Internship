fn reverse_string(s: &str) -> String {
    let mut reversed = String::new();
    for c in s.chars().rev() {
        reversed.push(c);
    }
    reversed
}

fn main() {
    let input_string = "Hello, World!";
    let reversed_string = reverse_string(input_string);
    println!("Reversed string: {}", reversed_string);
}
