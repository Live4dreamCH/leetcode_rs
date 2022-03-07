use std::cmp::Ordering;

/// 存储索引的单调栈
/// 用户需确保索引有效
pub struct MonoStack<'a, T: Ord> {
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
    pub fn new(array: &Vec<T>, order: Ordering, strict: bool) -> MonoStack<T> {
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
    pub fn push_once(&mut self, item: usize) -> Result<Option<usize>, usize> {
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
    pub fn push(&mut self, item: usize) {
        while let Err(_) = self.push_once(item) {}
    }

    pub fn pop(&mut self) -> Option<usize> {
        self.stack.pop()
    }

    pub fn top(&self) -> Option<&usize> {
        self.stack.last()
    }
}
