pub struct Solution {}

impl Solution {
    pub fn add_digits(num: i32) -> i32 {
        let mut num = num;
        let mut res = 0;
        loop {
            while num / 10 != 0 {
                res += num % 10;
                num /= 10;
            }
            res += num;
            if res / 10 == 0 {
                break;
            }
            num = res;
            res = 0;
        }
        res
    }
}

fn main() {
    let s = Solution::add_digits(38);
    assert_eq!(s, 2);
    dbg!(s);
    let s = Solution::add_digits(0);
    assert_eq!(s, 0);
    dbg!(s);
    let s = Solution::add_digits(1);
    assert_eq!(s, 1);
    dbg!(s);
}
