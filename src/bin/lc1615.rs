use std::collections::{HashMap, HashSet};

struct Solution;

impl Solution {
    pub fn maximal_network_rank(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        if roads.len() == 0 {
            return 0;
        }
        let mut road_to_map: HashMap<i32, HashSet<i32>> = HashMap::new();
        let mut count_map: HashMap<i32, i32> = HashMap::new();
        for road in roads {
            let from = road[0];
            let to = road[1];
            road_to_map.entry(from).or_default().insert(to);
            road_to_map.entry(to).or_default().insert(from);
            *count_map.entry(from).or_default() += 1;
            *count_map.entry(to).or_default() += 1;
        }
        let max_count = *count_map.iter().max_by_key(|kv| kv.1).unwrap().1;
        count_map.iter()
        1
    }
}
fn main() {
    Solution::maximal_network_rank(4, vec![vec![0, 1], vec![0, 3], vec![1, 2], vec![1, 3]]);
    Solution::maximal_network_rank(
        5,
        vec![
            vec![0, 1],
            vec![0, 3],
            vec![1, 2],
            vec![1, 3],
            vec![2, 3],
            vec![2, 4],
        ],
    );
    Solution::maximal_network_rank(
        8,
        vec![
            vec![0, 1],
            vec![1, 2],
            vec![2, 3],
            vec![2, 4],
            vec![5, 6],
            vec![5, 7],
        ],
    );
}
