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

    fn sift_up(&mut self, idx: usize) {
        // 如果当前节点不是根节点，且满足比较条件，则与父节点交换
        let mut current_idx = idx;
        let mut parent_idx = self.parent_idx(current_idx);
        
        while current_idx > 1 && (self.comparator)(&self.items[current_idx], &self.items[parent_idx]) {
            self.items.swap(current_idx, parent_idx);
            current_idx = parent_idx;
            parent_idx = self.parent_idx(current_idx);
        }
    }

    // 添加 sift_down 方法
    fn sift_down(&mut self, idx: usize) {
        // 如果当前节点有子节点
        if self.children_present(idx) {
            let smallest_child_idx = self.smallest_child_idx(idx);
            
            // 如果子节点满足比较条件，则与当前节点交换
            if (self.comparator)(&self.items[smallest_child_idx], &self.items[idx]) {
                self.items.swap(idx, smallest_child_idx);
                // 继续下沉
                self.sift_down(smallest_child_idx);
            }
        }
    }

    pub fn add(&mut self, value: T) {
        // 添加元素到堆的末尾
        self.items.push(value);
        self.count += 1;
        
        // 上浮操作，将新添加的元素移动到正确的位置
        self.sift_up(self.count);
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
        // 如果没有子节点，返回当前节点
        if !self.children_present(idx) {
            return idx;
        }
        
        let left_idx = self.left_child_idx(idx);
        let right_idx = self.right_child_idx(idx);
        
        // 如果只有左子节点，返回左子节点
        if right_idx > self.count {
            return left_idx;
        }
        
        // 返回满足比较条件的子节点
        if (self.comparator)(&self.items[left_idx], &self.items[right_idx]) {
            left_idx
        } else {
            right_idx
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
        // 如果堆为空，返回 None
        if self.is_empty() {
            return None;
        }
        
        // 取出堆顶元素
        let result = std::mem::replace(&mut self.items[1], T::default());
        
        // 将最后一个元素移动到堆顶
        if self.count > 1 {
            self.items[1] = self.items.pop().unwrap();
        } else {
            self.items.pop();
        }
        
        self.count -= 1;
        
        // 下沉操作，将堆顶元素移动到正确的位置
        if !self.is_empty() {
            self.sift_down(1);
        }
        
        Some(result)
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