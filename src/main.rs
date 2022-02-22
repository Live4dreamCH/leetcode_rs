mod lc717;

fn main() {
    let s = lc717::Solution::is_one_bit_character(vec![1, 0, 0]);
    assert_eq!(s, true);
    println!("{}", s);
    let s = lc717::Solution::is_one_bit_character(vec![1, 1, 1, 0]);
    assert_eq!(s, false);
    println!("{}", s);
}
