use std::cmp::min;

struct Solution;

impl Solution {
    pub fn moves_to_make_zigzag(nums: Vec<i32>) -> i32 {
        let mut even = 0;
        let mut odd = 0;
        for i in 0..nums.len() {
            let min = min(
                if 0 == i { i32::MAX } else { nums[i - 1] },
                *(nums.get(i + 1).unwrap_or(&i32::MAX)),
            );
            if nums[i] < min {
                continue;
            }
            if 0 == i % 2 {
                even += nums[i] - min + 1;
            } else {
                odd += nums[i] - min + 1;
            }
        }
        min(odd, even)
    }
}

fn main() {
    println!("{}", Solution::moves_to_make_zigzag(vec![1, 1, 1]));
    println!("{}", Solution::moves_to_make_zigzag(vec![9, 6, 1, 6, 2]));
}
