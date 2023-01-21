use std::collections::BTreeMap;

#[derive(Debug)]
pub struct OrderedMutiSet<T: Ord + Copy> {
    count_map: BTreeMap<T, u32>,
    length: usize,
}

impl<T: Ord + Copy> OrderedMutiSet<T> {
    pub fn new() -> Self {
        OrderedMutiSet {
            count_map: BTreeMap::new(),
            length: 0,
        }
    }
    pub fn push(&mut self, item: T) {
        *self.count_map.entry(item).or_insert(0) += 1;
        self.length += 1;
    }
    pub fn pop_min(&mut self) -> Option<T> {
        let key = *self.peek_min()?;
        self.remove_one(&key)
    }
    pub fn pop_max(&mut self) -> Option<T> {
        let key = *self.peek_max()?;
        self.remove_one(&key)
    }
    pub fn peek_min(&self) -> Option<&T> {
        self.count_map.iter().next().map(|p| p.0)
    }
    pub fn peek_max(&self) -> Option<&T> {
        self.count_map.iter().rev().next().map(|p| p.0)
    }
    pub fn len(&self) -> usize {
        self.length
    }
    pub fn remove_one(&mut self, item: &T) -> Option<T> {
        let count = self.count_map.get_mut(item)?;
        self.length -= 1;
        if *count > 1 {
            *count -= 1;
            Some(*item)
        } else {
            self.count_map.remove(item).map(|_| *item)
        }
    }
}

#[cfg(test)]
mod order_multi_set_test {
    use super::*;
    #[test]
    fn test_order_multi_set() {
        let mut set = OrderedMutiSet::new();
        // empty
        assert_eq!(set.len(), 0);
        assert_eq!(set.peek_min(), None);
        assert_eq!(set.peek_max(), None);
        assert_eq!(set.pop_min(), None);
        assert_eq!(set.pop_max(), None);
        assert_eq!(set.remove_one(&1), None);

        set.push(1);
        set.push(1);
        set.push(1);
        set.push(2);
        set.push(2);
        set.push(3);
        assert_eq!(set.len(), 6);
        assert_eq!(set.peek_min(), Some(&1));
        assert_eq!(set.peek_max(), Some(&3));
        assert_eq!(set.remove_one(&4), None);

        assert_eq!(set.remove_one(&2), Some(2));
        assert_eq!(set.len(), 5);

        assert_eq!(set.pop_max(), Some(3));
        assert_eq!(set.len(), 4);
        assert_eq!(set.peek_max(), Some(&2));

        assert_eq!(set.pop_min(), Some(1));
        assert_eq!(set.len(), 3);
        assert_eq!(set.peek_min(), Some(&1));
        assert_eq!(set.peek_max(), Some(&2));

        assert_eq!(set.remove_one(&1), Some(1));
        assert_eq!(set.len(), 2);
        assert_eq!(set.peek_min(), Some(&1));
        assert_eq!(set.peek_max(), Some(&2));

        assert_eq!(set.remove_one(&1), Some(1));
        assert_eq!(set.len(), 1);
        assert_eq!(set.peek_min(), Some(&2));
        assert_eq!(set.peek_max(), Some(&2));

        assert_eq!(set.pop_min(), Some(2));
        assert_eq!(set.len(), 0);
        assert_eq!(set.peek_min(), None);
        assert_eq!(set.peek_max(), None);

        assert_eq!(set.peek_min(), None);
        assert_eq!(set.peek_max(), None);
        assert_eq!(set.pop_min(), None);
        assert_eq!(set.pop_max(), None);
        assert_eq!(set.remove_one(&1), None);
        assert_eq!(set.len(), 0);
    }
}
