use std::collections::HashMap;
use std::collections::HashSet;
struct Solution;

impl Solution {
    pub fn get_folder_names(names: Vec<String>) -> Vec<String> {
        let mut name_times_map: HashMap<String, (HashSet<usize>, usize)> = HashMap::new();
        let mut actual_name_vec: Vec<String> = Vec::new();
        for name in &names {
            // 如果这个名字最终是a（k）形式，那就要先把k加到a的set里：
            
            // 此时如果a原来存k值是k-1的话，就要开始递加，加到set里找不到的第一个数为止
            |a: String, k: usize| {
                // 如果之前没存，就初始化
                if !name_times_map.contains_key(&a) {
                    name_times_map
                        .insert(a.clone(), (HashSet::from([k]), if 1 == k { 2 } else { 1 }));
                    // name_times_map.entry(a.clone()).or_insert((HashSet::from(k),if 1==k{2}else{1}));
                    return;
                }
                // 如果之前存了，就和之前的最大值比较
                // 如果小于，就报错
                if k < name_times_map.get(&a).unwrap().1 {
                    panic!("dup!")
                }
                while name_times_map
                    .get(&a)
                    .unwrap()
                    .0
                    .contains(&name_times_map.get(&a).unwrap().1)
                {
                    name_times_map.get_mut(&a).unwrap().1 += 1
                }
            };

            // 如果之前没用过，就直接用
            // 并把下次要补的k记录下来（一定为1）
            if !name_times_map.contains_key(name) {
                actual_name_vec.push(name.clone());
                name_times_map.insert(name.clone(), 1);
                continue;
            }
            // 如果这个名字之前已经用过了，就去查下这个名字之后应该补的是什么
            let actual_name = name.clone() + "(" + &name_times_map[name].to_string() + ")";
            actual_name_vec.push(actual_name.clone());
            *name_times_map.get_mut(name).unwrap() += 1;
            *name_times_map.entry(actual_name).or_insert(0) += 1;
        }
        actual_name_vec
    }
}
fn main() {
    assert_eq!(
        vec![
            String::from("pes"),
            String::from("fifa"),
            String::from("gta"),
            String::from("pes(2019)")
        ],
        Solution::get_folder_names(vec![
            String::from("pes"),
            String::from("fifa"),
            String::from("gta"),
            String::from("pes(2019)")
        ])
    );
    assert_eq!(
        vec![
            String::from("gta"),
            String::from("gta(1)"),
            String::from("gta(2)"),
            String::from("avalon")
        ],
        Solution::get_folder_names(vec![
            String::from("gta"),
            String::from("gta(1)"),
            String::from("gta"),
            String::from("avalon")
        ])
    );
    assert_eq!(
        vec![
            String::from("onepiece"),
            String::from("onepiece(1)"),
            String::from("onepiece(2)"),
            String::from("onepiece(3)"),
            String::from("onepiece(4)")
        ],
        Solution::get_folder_names(vec![
            String::from("onepiece"),
            String::from("onepiece(1)"),
            String::from("onepiece(2)"),
            String::from("onepiece(3)"),
            String::from("onepiece")
        ])
    );
    assert_eq!(
        vec![
            String::from("wano"),
            String::from("wano(1)"),
            String::from("wano(2)"),
            String::from("wano(3)")
        ],
        Solution::get_folder_names(vec![
            String::from("wano"),
            String::from("wano"),
            String::from("wano"),
            String::from("wano")
        ])
    );
    assert_eq!(
        vec![
            String::from("kaido"),
            String::from("kaido(1)"),
            String::from("kaido(2)"),
            String::from("kaido(1)(1)")
        ],
        Solution::get_folder_names(vec![
            String::from("kaido"),
            String::from("kaido(1)"),
            String::from("kaido"),
            String::from("kaido(1)")
        ])
    );
}
