pub struct Solution {}

fn str_to_complex(num: &str) -> (i32, i32) {
    for (i, c) in num.bytes().enumerate() {
        if c == b'+' {
            return (
                num[..i].parse().unwrap(),
                num[i + 1..num.len() - 1].parse().unwrap(),
            );
        }
    }
    (0, 0)
}

impl Solution {
    pub fn complex_number_multiply(num1: String, num2: String) -> String {
        let (re1, im1) = str_to_complex(&num1);
        let (re2, im2) = str_to_complex(&num2);
        let re = re1 * re2 - im1 * im2;
        let im = re1 * im2 + re2 * im1;
        format!("{}+{}i", re, im)
    }
}

fn main() {
    let s = Solution::complex_number_multiply("1+1i".to_string(), "1+1i".to_string());
    assert_eq!(s, "0+2i".to_string());
    dbg!(s);
    let s = Solution::complex_number_multiply("1+-1i".to_string(), "1+-1i".to_string());
    assert_eq!(s, "0+-2i".to_string());
    dbg!(s);
}
