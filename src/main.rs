mod lc553;
use lc553::Solution;

fn main() {
    let s = Solution::optimal_division(vec![1000, 100, 10, 2]);
    assert_eq!(s, "1000/(100/10/2)");
    dbg!(s);
}
