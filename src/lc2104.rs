mod algo;
use algo::MonoStack;
use std::cmp::Ordering;

pub struct Solution {}

impl Solution {
    pub fn sub_array_ranges(nums: Vec<i32>) -> i64 {
        let mut less_stack = MonoStack::new(&nums, Ordering::Less, false);
        let mut greater_stack = MonoStack::new(&nums, Ordering::Greater, true);
        let n = nums.len();
        let mut left_greater = vec![-1; n];
        let mut left_less = vec![-1; n];
        let mut right_greater = vec![n; n];
        let mut right_less = vec![n; n];
        for i in 0..n {
            if let Some(prev) = loop {
                match less_stack.push_once(i) {
                    Err(top) => right_less[top] = i,
                    Ok(p) => break p,
                }
            } {
                left_less[i] = prev as i32;
            }
            if let Some(prev) = loop {
                match greater_stack.push_once(i) {
                    Err(top) => right_greater[top] = i,
                    Ok(p) => break p,
                }
            } {
                left_greater[i] = prev as i32;
            }
        }
        let mut res = 0;
        for i in 0..n {
            res = res
                + nums[i] as i64
                    * ((i as i32 - left_greater[i]) * (right_greater[i] - i) as i32
                        - (i as i32 - left_less[i]) * (right_less[i] - i) as i32)
                        as i64;
        }
        res
    }
}

fn main() {
    let s = Solution::sub_array_ranges(vec![4, -2, -3, 4, 1]);
    assert_eq!(s, 59);
    dbg!(s);
    let s = Solution::sub_array_ranges(vec![1, 2, 3]);
    assert_eq!(s, 4);
    dbg!(s);
    let s = Solution::sub_array_ranges(vec![1, 3, 3]);
    assert_eq!(s, 4);
    dbg!(s);
}
