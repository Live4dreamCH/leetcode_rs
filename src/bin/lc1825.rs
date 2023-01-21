mod algo;
use algo::OrderedMutiSet;
use std::collections::VecDeque;

#[derive(Debug)]
struct MKAverage {
    m: i32,
    k: i32,
    queue: VecDeque<i32>,
    middle_set: OrderedMutiSet<i32>,
    small_set: OrderedMutiSet<i32>,
    large_set: OrderedMutiSet<i32>,
    sum_middle: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MKAverage {
    fn new(m: i32, k: i32) -> Self {
        MKAverage {
            m,
            k,
            queue: VecDeque::new(),
            middle_set: OrderedMutiSet::new(),
            small_set: OrderedMutiSet::new(),
            large_set: OrderedMutiSet::new(),
            sum_middle: 0,
        }
    }

    fn balance_sets(&mut self) {
        for _ in self.small_set.len()..self.k as usize {
            if let Some(pop_num) = self.middle_set.pop_min() {
                self.sum_middle -= pop_num;
                self.small_set.push(pop_num);
            } else {
                break;
            }
        }
        for _ in self.k as usize..self.small_set.len() {
            if let Some(pop_num) = self.small_set.pop_max() {
                self.sum_middle += pop_num;
                self.middle_set.push(pop_num);
            } else {
                break;
            }
        }
        for _ in self.large_set.len()..self.k as usize {
            if let Some(pop_num) = self.middle_set.pop_max() {
                self.sum_middle -= pop_num;
                self.large_set.push(pop_num);
            } else {
                break;
            }
        }
        for _ in self.k as usize..self.large_set.len() {
            if let Some(pop_num) = self.large_set.pop_min() {
                self.sum_middle += pop_num;
                self.middle_set.push(pop_num);
            } else {
                break;
            }
        }
    }

    fn add_element(&mut self, num: i32) {
        // 向队列尾添加num
        // 和两个阈值比较，决定num落入哪个集合中（可能膨胀超长）
        // 如果队列已超长，从队头弹出一个元素
        // 在三个集合中寻找弹出元素，并弹出
        //  先在中间集合中寻找，如果不存在，则继续在两端集合中寻找
        // 由于以上的一次插入和可能的一次删除，三个集合的长度可能不再满足题意。
        // 在相邻集合间移动元素，使集合长度重新平衡
        // 注意修改中间集合时同步维护sum值
        self.queue.push_back(num);
        if num <= *self.small_set.peek_max().unwrap_or(&i32::MIN) {
            self.small_set.push(num);
        } else if num >= *self.large_set.peek_min().unwrap_or(&i32::MAX) {
            self.large_set.push(num);
        } else {
            self.middle_set.push(num);
            self.sum_middle += num;
        }

        if self.queue.len() > self.m as usize {
            if let Some(pop_num) = self.queue.pop_front() {
                match self.middle_set.remove_one(&pop_num) {
                    Some(rm_num) => self.sum_middle -= rm_num,
                    None => {
                        self.small_set.remove_one(&pop_num);
                        self.large_set.remove_one(&pop_num);
                    }
                }
            }
        }
        if self.queue.len() >= self.m as usize {
            self.balance_sets();
        }
    }

    fn calculate_mk_average(&self) -> i32 {
        if self.queue.len() < self.m as usize {
            return -1;
        }
        self.sum_middle / (self.m - 2 * self.k)
    }
}

/**
 * Your MKAverage object will be instantiated and called as such:
 * let obj = MKAverage::new(m, k);
 * obj.add_element(num);
 * let ret_2: i32 = obj.calculate_mk_average();
 */

#[cfg(test)]
mod lc1825_test {
    use std::collections::HashMap;

    use crate::MKAverage;
    #[test]
    fn test_mk_average() {
        let mut obj = MKAverage::new(3, 1);
        obj.add_element(3); // 当前元素为 [3]
        obj.add_element(1); // 当前元素为 [3,1]
        assert_eq!(obj.calculate_mk_average(), -1); // 返回 -1 ，因为 m = 3 ，但数据流中只有 2 个元素
        obj.add_element(10); // 当前元素为 [3,1,10]
        assert_eq!(obj.calculate_mk_average(), 3); // 最后 3 个元素为 [3,1,10]
                                                   // 删除最小以及最大的 1 个元素后，容器为 [3]
                                                   // [3] 的平均值等于 3/1 = 3 ，故返回 3
        obj.add_element(5); // 当前元素为 [3,1,10,5]
        obj.add_element(5); // 当前元素为 [3,1,10,5,5]
        obj.add_element(5); // 当前元素为 [3,1,10,5,5,5]
        assert_eq!(obj.calculate_mk_average(), 5); // 最后 3 个元素为 [5,5,5]
                                                   // 删除最小以及最大的 1 个元素后，容器为 [5]
                                                   // [5] 的平均值等于 5/1 = 5 ，故返回 5

        obj = MKAverage::new(3, 1);
        let mut call_fn_by = |name: &str, num: Option<i32>| -> Option<i32> {
            match name {
                "addElement" => {
                    obj.add_element(num.unwrap());
                    None
                }
                "calculateMKAverage" => Some(obj.calculate_mk_average()),
                _ => panic!(),
            }
        };

        let names = [
            "MKAverage",
            "addElement",
            "addElement",
            "calculateMKAverage",
            "addElement",
            "addElement",
            "calculateMKAverage",
            "addElement",
            "addElement",
            "calculateMKAverage",
            "addElement",
        ];
        let params = [
            Some(3),
            Some(17612),
            Some(74607),
            None,
            Some(8272),
            Some(33433),
            None,
            Some(15456),
            Some(64938),
            None,
            Some(99741),
        ];
        let res = [
            None,
            None,
            None,
            Some(-1),
            None,
            None,
            Some(33433),
            None,
            None,
            Some(33433),
            None,
        ];
        for i in 1..names.len() {
            assert_eq!(call_fn_by(names[i], params[i]), res[i]);
        }
    }
}
fn main() {}
