/*
	heap
	This question requires you to implement a binary heap function
*/
//本题需要实现二叉堆（Binary Heap） 的核心功能，具体包括：


//这个没有细看，后面按需学习
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

    // 1. 补全 add 方法：插入元素并“上浮”调整堆结构
    pub fn add(&mut self, value: T) {
        // 步骤1：把新元素加入数组末尾（对应完全二叉树的最后一个节点）
        self.items.push(value);
        self.count += 1;

        // 步骤2：“上浮”调整：让新元素向上移动，直到满足堆规则
        let mut current_idx = self.count; // 新元素的初始索引（数组最后一位）
        while current_idx > 1 { // 根节点（索引1）无需上浮
            let parent_idx = self.parent_idx(current_idx);
            // 比较当前节点和父节点：若当前节点更符合堆规则（如最小堆中更小），则交换
            if (self.comparator)(&self.items[current_idx], &self.items[parent_idx]) {
                self.items.swap(current_idx, parent_idx);
                current_idx = parent_idx; // 继续向上比较父节点
            } else {
                break; // 满足堆规则，停止上浮
            }
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

    // 2. 补全 smallest_child_idx 方法：找到当前节点的两个子节点中“更符合堆规则”的索引
    fn smallest_child_idx(&self, idx: usize) -> usize {
        let left_idx = self.left_child_idx(idx);
        let right_idx = self.right_child_idx(idx);

        // 情况1：只有左子节点（右子节点超出元素个数）
        if right_idx > self.count {
            return left_idx;
        }

        // 情况2：有两个子节点，比较哪个更符合堆规则（如最小堆选更小的，最大堆选更大的）
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
        // 情况1：堆为空，返回 None
        if self.is_empty() {
            return None;
        }

        // 步骤1：弹出堆顶元素（索引1，数组第一个有效元素）
        let top_item = self.items.swap_remove(1); // swap_remove(1)：删除索引1并返回元素，用最后一个元素填充索引1
        self.count -= 1;

        // 步骤2：“下沉”调整：让新的堆顶元素（原最后一个元素）向下移动，直到满足堆规则
        let mut current_idx = 1;
        while self.children_present(current_idx) { // 有子节点时才需要下沉
            let child_idx = self.smallest_child_idx(current_idx);
            // 比较当前节点和子节点：若子节点更符合堆规则，则交换
            if (self.comparator)(&self.items[child_idx], &self.items[current_idx]) {
                self.items.swap(current_idx, child_idx);
                current_idx = child_idx; // 继续向下比较子节点
            } else {
                break; // 满足堆规则，停止下沉
            }
        }

        // 返回弹出的堆顶元素
        Some(top_item)
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