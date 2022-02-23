mod lc917;
use lc917::Solution;

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
