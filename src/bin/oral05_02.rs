use std::fmt::Binary;

struct Solution;

impl Solution {
    pub fn print_bin(num: f64) -> String {
        let mut bin_str = String::from("0.");
        let mut decimal = num;
        while decimal != 0.0 {
            if bin_str.len() >= 32 {
                return String::from("ERROR");
            }
            decimal *= 2.0;
            if decimal >= 1.0 {
                bin_str += "1";
                decimal -= 1.0;
            } else {
                bin_str += "0";
            }
        }
        bin_str
    }
}

fn main() {
    assert_eq!(Solution::print_bin(0.625), String::from("0.101"));
    assert_eq!(Solution::print_bin(0.1), String::from("ERROR"));
}
