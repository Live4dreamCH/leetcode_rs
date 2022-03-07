use std::cmp::Ordering;

/// 存储索引的单调栈
/// 用户需确保索引有效
struct MonoStack<'a, T: Ord> {
    /// 栈本体，存储索引
    stack: Vec<usize>,
    /// 原数组的引用，用于比较数值大小
    array: &'a [T],
    /// 单调栈是递增或递减
    ///
    /// Less: bottom[1,2,3]top
    ///
    /// Greater: bottom[3,2,1]top
    order: Ordering,
    /// 是否严格单调
    strict: bool,
}

impl<'a, T: Ord> MonoStack<'a, T> {
    fn new(array: &Vec<T>, order: Ordering, strict: bool) -> MonoStack<T> {
        MonoStack {
            stack: Vec::new(),
            array,
            order,
            strict,
        }
    }

    /// 尝试执行一次push
    ///
    /// 若成功，返回Ok(Some(前一个索引)或None)
    ///
    /// 若失败，进行一次pop，返回Err(usize)
    fn push_once(&mut self, item: usize) -> Result<Option<usize>, usize> {
        if let Some(&top) = self.top() {
            let o = self.array[top].cmp(&self.array[item]);
            if o == self.order {
                self.stack.push(item);
                Result::Ok(Some(top))
            } else if o == Ordering::Equal {
                if self.strict {
                    Result::Err(self.pop().unwrap())
                } else {
                    self.stack.push(item);
                    Result::Ok(Some(top))
                }
            } else {
                Result::Err(self.pop().unwrap())
            }
        } else {
            self.stack.push(item);
            Result::Ok(None)
        }
    }
    /// push直到成功为止
    fn push(&mut self, item: usize) {
        while let Err(_) = self.push_once(item) {}
    }

    fn pop(&mut self) -> Option<usize> {
        self.stack.pop()
    }

    fn top(&self) -> Option<&usize> {
        self.stack.last()
    }
}

pub struct Solution {}

impl Solution {
    pub fn sub_array_ranges(nums: Vec<i32>) -> i64 {
        let mut less_stack = MonoStack::new(&nums, Ordering::Less, false);
        let mut greater_stack = MonoStack::new(&nums, Ordering::Greater, true);
        let n = nums.len();
        let mut left_greater = vec![-1; n];
        let mut left_less = vec![-1; n];
        let mut right_greater = vec![n; n];
        let mut right_less = vec![n; n];
        for i in 0..n {
            if let Some(prev) = loop {
                match less_stack.push_once(i) {
                    Err(top) => right_less[top] = i,
                    Ok(p) => break p,
                }
            } {
                left_less[i] = prev as i32;
            }
            if let Some(prev) = loop {
                match greater_stack.push_once(i) {
                    Err(top) => right_greater[top] = i,
                    Ok(p) => break p,
                }
            } {
                left_greater[i] = prev as i32;
            }
        }
        let mut res = 0;
        for i in 0..n {
            res = res
                + nums[i] as i64
                    * ((i as i32 - left_greater[i]) * (right_greater[i] - i) as i32
                        - (i as i32 - left_less[i]) * (right_less[i] - i) as i32)
                        as i64;
        }
        res
    }
}

fn main() {
    let s = Solution::sub_array_ranges(vec![4, -2, -3, 4, 1]);
    assert_eq!(s, 59);
    dbg!(s);
    let s = Solution::sub_array_ranges(vec![1, 2, 3]);
    assert_eq!(s, 4);
    dbg!(s);
    let s = Solution::sub_array_ranges(vec![1, 3, 3]);
    assert_eq!(s, 4);
    dbg!(s);
}
