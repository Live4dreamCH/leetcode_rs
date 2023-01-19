pub struct Solution {}

impl Solution {
    pub fn find_lu_slength(a: String, b: String) -> i32 {
        if a.len() != b.len() {
            if a.len() > b.len() {
                return a.len() as i32;
            } else {
                return b.len() as i32;
            }
        }
        if a != b {
            return a.len() as i32;
        }
        return -1;
    }
}

fn main() {
    let s = Solution::find_lu_slength("aba".to_string(), "cdc".to_string());
    assert_eq!(s, 3);
    dbg!(s);
    let s = Solution::find_lu_slength("aaa".to_string(), "bbb".to_string());
    assert_eq!(s, 3);
    dbg!(s);
    let s = Solution::find_lu_slength("aaa".to_string(), "aaa".to_string());
    assert_eq!(s, -1);
    dbg!(s);
}
