mod lc172;
use lc172::Solution;

trait A {
    fn aa(&self);
    fn aba(&self);
}

trait B {
    fn bb(&self);
    fn aba(&self);
}

// 既实现A又实现B？
struct AB {}

fn main() {
    let in_out = [
        (0, 0),
        (4, 0),
        (5, 1),
        (9, 1),
        (10, 2),
        (14, 2),
        (15, 3),
        (19, 3),
        (20, 4),
        (24, 4),
        (25, 6),
        (29, 6),
    ];
    for io in in_out {
        let s = Solution::trailing_zeroes(io.0);
        assert_eq!(s, io.1);
    }
}
