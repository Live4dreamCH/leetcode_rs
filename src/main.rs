mod lc2016;
use lc2016::Solution;

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
