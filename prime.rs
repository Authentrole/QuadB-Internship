use std::io;

fn is_prime(num: u32) -> bool {
    if num <= 1 {
        return false;
    }
    let mut i = 2;
    while i * i <= num {
        if num % i == 0 {
            return false;
        }
        i += 1;
    }
    true
}

fn main() {
    println!("Enter a number:");
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let num: u32 = input.trim().parse().expect("Invalid input");

    if is_prime(num) {
        println!("{} is a prime number", num);
    } else {
        println!("{} is not a prime number", num);
    }
}
