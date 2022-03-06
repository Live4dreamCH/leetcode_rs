pub struct Solution {}

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }
        let round = num_rows + num_rows - 2;
        let mut res = String::new();
        let s = s.into_bytes();
        for left in 0..round / 2 + 1 {
            let right = round - left;
            let mut i = left as usize;
            let mut j = right as usize;
            while i < s.len() {
                res.push(s[i] as char);
                i = i + round as usize;
                if left != 0 && left != right && j < s.len() {
                    res.push(s[j] as char);
                    j = j + round as usize;
                }
            }
        }
        res
    }
}

fn main() {
    let s = Solution::convert("PAYPALISHIRING".to_string(), 3);
    assert_eq!(s, "PAHNAPLSIIGYIR".to_string());
    dbg!(s);
    let s = Solution::convert("PAYPALISHIRING".to_string(), 4);
    assert_eq!(s, "PINALSIGYAHRPI".to_string());
    dbg!(s);
    let s = Solution::convert("A".to_string(), 1);
    assert_eq!(s, "A".to_string());
    dbg!(s);
}
