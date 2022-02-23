pub struct Solution {}
impl Solution {
    pub fn reverse_only_letters(s: String) -> String {
        let mut s = s.into_bytes();
        let mut i = 0;
        let mut j = s.len() - 1;
        while i < j {
            if (b'a' <= s[i] && s[i] <= b'z') || (b'A' <= s[i] && s[i] <= b'Z') {
                while i < j {
                    if (b'a' <= s[j] && s[j] <= b'z') || (b'A' <= s[j] && s[j] <= b'Z') {
                        let temp = s[i];
                        s[i] = s[j];
                        s[j] = temp;
                        j = j - 1;
                        break;
                    }
                    j = j - 1;
                }
            }
            i = i + 1;
        }
        String::from_utf8(s).unwrap()
    }
}

fn main() {
    let s = Solution::reverse_only_letters("ab-cd".to_string());
    assert_eq!(s, "dc-ba".to_string());
    println!("{}", s);
    let s = Solution::reverse_only_letters("a-bC-dEf-ghIj".to_string());
    assert_eq!(s, "j-Ih-gfE-dCba".to_string());
    println!("{}", s);
    let s = Solution::reverse_only_letters("Test1ng-Leet=code-Q!".to_string());
    assert_eq!(s, "Qedo1ct-eeLg=ntse-T!".to_string());
    println!("{}", s);
}
