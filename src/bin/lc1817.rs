use std::collections::{HashMap, HashSet};

struct Solution;

impl Solution {
    pub fn finding_users_active_minutes(logs: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let mut id_times_map: HashMap<i32, HashSet<i32>> = HashMap::new();
        for log in logs {
            let (id, time) = if let [id, time, ..] = log[..] {
                (id, time)
            } else {
                todo!()
            };
            id_times_map.entry(id).or_default().insert(time);
        }
        let mut answer = vec![0; k as usize];
        for id_times in id_times_map {
            let time = id_times.1.len();
            if 1 <= time && time <= k as usize {
                answer[time - 1] += 1;
            }
        }
        answer
    }
}

fn main() {}
