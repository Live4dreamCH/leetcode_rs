struct Solution;
impl Solution {
    pub fn minimum_deletions(s: String) -> i32 {
        let mut del: Vec<i32> = vec![0; s.len() + 1];
        let mut left_b = 0;
        let mut i = 0;
        for c in s.bytes() {
            i += 1;
            if c == b'b' {
                left_b += 1;
            }
            del[i] = left_b;
        }
        i = s.len();
        let mut right_a = 0;
        for c in s.bytes().rev() {
            i -= 1;
            if c == b'a' {
                right_a += 1;
            }
            del[i] += right_a;
        }
        *del.iter().min().unwrap()
    }
}
fn main() {
    assert_eq!(2, Solution::minimum_deletions(String::from("aababbab")));
    assert_eq!(2, Solution::minimum_deletions(String::from("bbaaaaabb")));
}
