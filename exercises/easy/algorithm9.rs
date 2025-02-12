/*
	heap
	This question requires you to implement a binary heap function
*/


use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        // 直接作为数组的最后一个元素加入
        let last_idx = self.count;
        let last_mut_item = self.items.get_mut(last_idx);
        match last_mut_item {
            Some(e) => {
                *e = value;
            },
            None => {
              self.items.push(value);  
            },
        };
        self.count += 1;
        // 堆化
        let mut idx = last_idx;
        while idx != 0 {
            
            let parent_idx = self.parent_idx(idx);
            
            if (self.comparator)(self.items.get(parent_idx).unwrap() , self.items.get(idx).unwrap()) {
                // 满足堆的定义
                break;
            }
            // 交换
            self.items.swap(idx, parent_idx);
            idx = parent_idx;
        }
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        //
        let left_idx = self.left_child_idx(idx);
        let right_idx = self.right_child_idx(idx);
        if left_idx >= self.count && right_idx >= self.count {
            return idx;
        }else if left_idx >= self.count {
            let child = self.items.get(right_idx).unwrap();
            let val = self.items.get(idx).unwrap();
            if (self.comparator)(child, val) {
                return right_idx;
            }else {
                return idx;
            }
        }else if right_idx >= self.count {
            let child = self.items.get(left_idx).unwrap();
            let val = self.items.get(idx).unwrap();
            if (self.comparator)(child, val) {
                return left_idx;
            }else {
                return idx;
            }
        }else {
            let left_child = self.items.get(left_idx).unwrap();
            let right_child = self.items.get(right_idx).unwrap();
            let val = self.items.get(idx).unwrap();

            if (self.comparator)(left_child,right_child) {
                if (self.comparator)(left_child, val) {
                    return left_idx;
                }else {
                    return idx;
                }
            }else {
                if (self.comparator)(right_child, val) {
                    return right_idx;
                }else {
                    return idx;
                }
            }         
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        //
        if self.is_empty() {
		    return None;
        }
        // 将堆顶元素和第一个元素进行交换
        self.items.swap(0, self.count - 1);
        // 拿出最后一个元素
        let res = self.items.pop();
        self.count -= 1;
        // 将堆顶的元素进行从上到下的堆化
        {    
            let mut idx = 0;
            while idx < self.count {
                let child_idx = self.smallest_child_idx(idx);
                if idx == child_idx {
                    break;
                }else {
                    // 交换元素
                    self.items.swap(idx, child_idx);
                    idx = child_idx;
                }
            }
        }    
        res
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}