pub struct Solution {}
impl Solution {
    pub fn is_one_bit_character(bits: Vec<i32>) -> bool {
        let mut i = 0;
        let mut res = true;
        while i < bits.len() {
            match bits[i] {
                0 => {
                    i = i + 1;
                    res = true;
                }
                1 => {
                    i = i + 2;
                    res = false;
                }
                _ => (),
            }
        }
        res
    }
}
