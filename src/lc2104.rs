// use std::cmp::Ordering;
// #[derive(Debug)]
// struct MonotoneStack {
//     // store the index
//     stack: Vec<usize>,
//     // Less: bottom[1,2,3]top
//     // Greater: bottom[3,2,1]top
//     order: Ordering,
//     array:
// }
// impl<T> MonotoneStack<T> {
//     fn new(order: Ordering) -> MonotoneStack<T> {
//         MonotoneStack {
//             stack: Vec::new(),
//             order,
//         }
//     }
//     fn push(&mut self, item: T) -> Result<(), ()> {
//         if
//         self.stack.push(item)
//     }
// }

pub struct Solution {}

impl Solution {
    pub fn sub_array_ranges(nums: Vec<i32>) -> i64 {
        let mut less_stack = Vec::<usize>::new();
        let mut greater_stack = Vec::<usize>::new();
        let n = nums.len();
        let mut left_greater = vec![-1; n];
        let mut left_less = vec![-1; n];
        let mut right_greater = vec![n; n];
        let mut right_less = vec![n; n];
        for i in 0..n {
            while let Some(&top) = less_stack.last() {
                if nums[top] > nums[i] {
                    right_less[top] = i;
                    less_stack.pop();
                } else {
                    break;
                }
            }
            if let Some(&top) = less_stack.last() {
                left_less[i] = top as i32;
            }
            less_stack.push(i);

            while let Some(&top) = greater_stack.last() {
                if nums[top] <= nums[i] {
                    right_greater[top] = i;
                    greater_stack.pop();
                } else {
                    break;
                }
            }
            if let Some(&top) = greater_stack.last() {
                left_greater[i] = top as i32;
            }
            greater_stack.push(i);
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
