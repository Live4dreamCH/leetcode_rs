pub struct Solution {}

impl Solution {
    pub fn convert_to_base7(num: i32) -> String {
        if num == 0 {
            return "0".to_string();
        }
        let mut n;
        let mut sign = true;
        if num < 0 {
            n = -num;
            sign = false;
        } else {
            n = num;
        }
        let mut res = String::new();
        while n != 0 {
            res = (n % 7).to_string() + &res;
            n /= 7;
        }
        if sign == false {
            res = '-'.to_string() + &res;
        }
        res
    }
}

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
}
