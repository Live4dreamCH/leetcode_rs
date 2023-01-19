pub struct Solution {}

impl Solution {
    pub fn maximum_requests(n: i32, requests: Vec<Vec<i32>>) -> i32 {
        let mut requests = requests;
        let mut stack = Vec::new();
        let mut visited = vec![requests[0][0]];
        stack.push(requests[0][0]);
        while !stack.is_empty() {
            for edge in &requests {
                if edge[0] == stack.pop().unwrap() {}
            }
        }
        1
    }
}

fn main() {
    let s = Solution::maximum_requests(
        5,
        vec![
            vec![0, 1],
            vec![1, 0],
            vec![0, 1],
            vec![1, 2],
            vec![2, 0],
            vec![3, 4],
        ],
    );
    assert_eq!(s, 5);
    dbg!(s);
}
