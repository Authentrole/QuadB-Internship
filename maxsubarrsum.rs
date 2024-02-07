fn max_subarray_sum(nums: &[i32]) -> i32 {
    let mut max_sum = std::i32::MIN;
    let mut current_sum = 0;

    for &num in nums {
        current_sum = current_sum.max(0) + num; // Take the maximum between the current sum and 0, then add the current number
        max_sum = max_sum.max(current_sum); // Update the maximum sum
    }

    max_sum
}

fn main() {
    let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
    let max_sum = max_subarray_sum(&nums);
    println!("Maximum subarray sum: {}", max_sum);
}
