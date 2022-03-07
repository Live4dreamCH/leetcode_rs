mod lc2104;
use lc2104::Solution;

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
