use std::io;

fn find_first_occurrence(arr: &[i32], target: i32) -> Option<usize> {
    let mut low = 0;
    let mut high = arr.len() - 1;
    let mut result = None;

    while low <= high {
        let mid = low + (high - low) / 2;
        if arr[mid] == target {
            result = Some(mid);
            high = mid - 1; // continue searching in the left half for the first occurrence
        } else if arr[mid] < target {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }

    result
}

fn main() {
    println!("Enter the sorted array (space-separated):");
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let arr: Vec<i32> = input.split_whitespace()
        .map(|x| x.parse().expect("Invalid input"))
        .collect();

    println!("Enter the target number:");
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let target: i32 = input.trim().parse().expect("Invalid input");

    if let Some(index) = find_first_occurrence(&arr, target) {
        println!("First occurrence of {} is at index {}", target, index);
    } else {
        println!("{} not found in the array", target);
    }
}
