pub struct Solution {}

impl Solution {
    pub fn maximum_difference(nums: Vec<i32>) -> i32 {
        let mut min = 1e9 as i32 + 5;
        let mut diff = -1;
        for num in nums {
            if num < min {
                min = num;
            } else if num - min > diff {
                diff = num - min;
            }
        }
        if diff == 0 {
            -1
        } else {
            diff
        }
    }
}

fn main() {
    let s = Solution::maximum_difference(vec![7, 1, 5, 4]);
    assert_eq!(s, 4);
    dbg!(s);
    let s = Solution::maximum_difference(vec![9, 4, 3, 2]);
    assert_eq!(s, -1);
    dbg!(s);
    let s = Solution::maximum_difference(vec![1, 5, 2, 10]);
    assert_eq!(s, 9);
    dbg!(s);
}
