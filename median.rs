use std::io;

fn median(arr: &[i32]) -> Option<f64> {
    let len = arr.len();
    if len == 0 {
        return None;
    }

    if len % 2 == 0 {
        // If the length of the array is even, return the average of the middle two elements
        let mid_right = len / 2;
        let mid_left = mid_right - 1;
        Some((arr[mid_left] + arr[mid_right]) as f64 / 2.0)
    } else {
        // If the length of the array is odd, return the middle element
        let mid = len / 2;
        Some(arr[mid] as f64)
    }
}

fn main() {
    println!("Enter a sorted array of integers (space-separated):");
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let arr: Vec<i32> = input.trim().split_whitespace()
        .map(|x| x.parse().expect("Invalid input"))
        .collect();

    if let Some(median_val) = median(&arr) {
        println!("The median of the array is: {}", median_val);
    } else {
        println!("The array is empty.");
    }
}
