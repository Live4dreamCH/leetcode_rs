mod lc1601;
use lc1601::Solution;

fn main() {
    let s = Solution::maximum_requests(
        5,
        vec![
            vec![0, 1],
            vec![1, 0],
            vec![0, 1],
            vec![1, 2],
            vec![2, 0],
            vec![3, 4],
        ],
    );
    assert_eq!(s, 5);
    dbg!(s);
}
