use std::cmp::min;

struct Solution;
impl Solution {
    pub fn min_side_jumps(obstacles: Vec<i32>) -> i32 {
        let n = obstacles.len() - 1;
        let mut cost = [1, 0, 1];
        for i in 1..=n {
            let mut curr_cost = [i32::MAX; 3];
            for (j, v) in curr_cost.iter_mut().enumerate() {
                if obstacles[i] == 1 + j as i32 {
                    continue;
                }
                *v = cost[j];
            }
            let curr_min = *curr_cost.iter().min().unwrap_or(&i32::MAX);
            for (j, v) in curr_cost.iter_mut().enumerate() {
                if obstacles[i] == 1 + j as i32 {
                    continue;
                }
                *v = min(*v, curr_min + 1);
            }
            cost = curr_cost;
            // dbg!((i, &cost));
        }
        *cost.iter().min().unwrap_or(&0)
    }
}

fn main() {
    assert_eq!(Solution::min_side_jumps(vec![0, 1, 2, 3, 0]), 2);
    assert_eq!(Solution::min_side_jumps(vec![0, 1, 1, 3, 3, 0]), 0);
    assert_eq!(Solution::min_side_jumps(vec![0, 2, 1, 0, 3, 0]), 2);
    let x = i32::MAX;
    vec![
        vec![[1, 0, 1], [x, 0, 1], [2, x, 1], [2, 3, x], [2, 3, 4]],
        vec![[1, 0, 1], [x, 0, 1]],
        vec![
            [1, 0, 1],
            [1, x, 1],
            [x, 2, 1],
            [2, 2, 1],
            [2, 2, x],
            [2, 2, 3],
        ],
    ];
}
