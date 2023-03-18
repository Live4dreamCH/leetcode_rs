use std::collections::BTreeMap;

struct Solution;

impl Solution {
    pub fn merge_similar_items(items1: Vec<Vec<i32>>, items2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut value_weight_map: BTreeMap<i32, i32> = BTreeMap::new();
        let mut add = |items: Vec<Vec<i32>>| {
            for item in items {
                *value_weight_map.entry(item[0]).or_insert(0) += item[1];
            }
        };
        add(items1);
        add(items2);
        value_weight_map
            .into_iter()
            .map(|v_w| vec![v_w.0, v_w.1])
            .collect()
    }
}

fn main() {
    assert_eq!(
        vec![vec![1, 6], vec![3, 9], vec![4, 5]],
        Solution::merge_similar_items(
            vec![vec![1, 1], vec![4, 5], vec![3, 8]],
            vec![vec![3, 1], vec![1, 5]]
        )
    );
    assert_eq!(
        vec![vec![1, 4], vec![2, 4], vec![3, 4]],
        Solution::merge_similar_items(
            vec![vec![1, 1], vec![3, 2], vec![2, 3]],
            vec![vec![2, 1], vec![3, 2], vec![1, 3]]
        )
    );
    assert_eq!(
        vec![vec![1, 7], vec![2, 4], vec![7, 1]],
        Solution::merge_similar_items(
            vec![vec![1, 3], vec![2, 2]],
            vec![vec![7, 1], vec![2, 2], vec![1, 4]]
        )
    );
}
