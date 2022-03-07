mod lc504;
use lc504::Solution;

fn main() {
    let s = Solution::convert_to_base7(100);
    assert_eq!(s, "202");
    dbg!(s);
    let s = Solution::convert_to_base7(-7);
    assert_eq!(s, "-10");
    dbg!(s);
    let s = Solution::convert_to_base7(0);
    assert_eq!(s, "0");
    dbg!(s);
    let s = Solution::convert_to_base7(-8);
    assert_eq!(s, "-11");
    dbg!(s);
}
