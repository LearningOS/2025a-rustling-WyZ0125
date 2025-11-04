/*
    single linked list merge
    This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/

//再看时候要考虑语法细节
use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;
use std::vec::*;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node { val: t, next: None }
    }
}
#[derive(Debug)]
struct LinkedList<T> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

//作用：支持let list: LinkedList<i32> = Default::default();创建空链表，简化空链表初始化。
impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }

    pub fn add(&mut self, obj: T) {
        // 1. 创建新节点：用Box包装（自动管理内存），再转成原始指针
        let mut node = Box::new(Node::new(obj));
        node.next = None; // 确保新节点无后续节点
                          // Box::into_raw：将Box的所有权转为原始指针（后续需手动释放，避免内存泄漏）
                          // NonNull::new_unchecked：将原始指针转为NonNull（需确保指针非空，此处Box创建的节点必非空）
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });

        // 2. 将新节点接入链表尾部
        match self.end {
            None => self.start = node_ptr, // 空链表：新节点既是头也是尾
            Some(end_ptr) => unsafe {
                // 非空链表：尾节点的next指向新节点
                (*end_ptr.as_ptr()).next = node_ptr;
            },
        }

        // 3. 更新链表属性
        self.end = node_ptr; // 尾指针指向新节点
        self.length += 1; // 长度+1
    }

    //作用：对外提供 “按索引查值” 的接口，隐藏内部遍历逻辑。
    pub fn get(&mut self, index: i32) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }

    //从指定节点开始，递归遍历到第 index 个节点，返回其值（或 None 表示索引无效）
    fn get_ith_node(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }


    //核心算法:只有这部分是独特的
    pub fn merge(mut list_a: LinkedList<T>, mut list_b: LinkedList<T>) -> Self
    where
        T: Ord,
    {
        let mut merged = LinkedList::<T>::new(); // 创建空的合并后链表

        // 1. 提取两个链表的头指针（take()：取出Option中的值，原链表的start设为None，避免指针重复引用）
        let mut ptr_a = list_a.start.take();
        let mut ptr_b = list_b.start.take();

        // 2. 双指针遍历：每次选值较小的节点接入合并链表
        while let (Some(a), Some(b)) = (ptr_a, ptr_b) {
            unsafe {
                if (*a.as_ptr()).val <= (*b.as_ptr()).val {
                    // 选list_a的当前节点：更新ptr_a为下一个节点，清空当前节点的next，接入合并链表
                    ptr_a = (*a.as_ptr()).next;
                    (*a.as_ptr()).next = None;
                    merged.append_existing_node(a);
                } else {
                    // 选list_b的当前节点：逻辑同上
                    ptr_b = (*b.as_ptr()).next;
                    (*b.as_ptr()).next = None;
                    merged.append_existing_node(b);
                }
            }
        }

        // 3. 接入剩余节点（其中一个链表已遍历完，另一个还有剩余）
        while let Some(a) = ptr_a {
            unsafe {
                ptr_a = (*a.as_ptr()).next;
                (*a.as_ptr()).next = None;
                merged.append_existing_node(a);
            }
        }
        while let Some(b) = ptr_b {
            unsafe {
                ptr_b = (*b.as_ptr()).next;
                (*b.as_ptr()).next = None;
                merged.append_existing_node(b);
            }
        }

        merged // 返回合并后的链表
    }

    //作用：专门用于merge方法，将 “已存在的节点指针” 接入合并链表尾部，避免重复写 “接入尾部” 的逻辑。
    /// Helper function that appends an existing node pointer to the list
    fn append_existing_node(&mut self, node_ptr: NonNull<Node<T>>) {
        match self.end {
            None => {
                self.start = Some(node_ptr);
                self.end = Some(node_ptr);
            }
            Some(end_ptr) => unsafe {
                (*end_ptr.as_ptr()).next = Some(node_ptr);
                self.end = Some(node_ptr);
            },
        }
        self.length += 1;
    }
}

impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.start {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.next {
            Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.val),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        println!("Linked List is {}", list);
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        list_str.add("C".to_string());
        println!("Linked List is {}", list_str);
        assert_eq!(3, list_str.length);
    }

    #[test]
    fn test_merge_linked_list_1() {
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_a = vec![1, 3, 5, 7];
        let vec_b = vec![2, 4, 6, 8];
        let target_vec = vec![1, 2, 3, 4, 5, 6, 7, 8];

        for i in 0..vec_a.len() {
            list_a.add(vec_a[i]);
        }
        for i in 0..vec_b.len() {
            list_b.add(vec_b[i]);
        }
        println!("list a {} list b {}", list_a, list_b);
        let mut list_c = LinkedList::<i32>::merge(list_a, list_b);
        println!("merged List is {}", list_c);
        for i in 0..target_vec.len() {
            assert_eq!(target_vec[i], *list_c.get(i as i32).unwrap());
        }
    }
    #[test]
    fn test_merge_linked_list_2() {
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_a = vec![11, 33, 44, 88, 89, 90, 100];
        let vec_b = vec![1, 22, 30, 45];
        let target_vec = vec![1, 11, 22, 30, 33, 44, 45, 88, 89, 90, 100];

        for i in 0..vec_a.len() {
            list_a.add(vec_a[i]);
        }
        for i in 0..vec_b.len() {
            list_b.add(vec_b[i]);
        }
        println!("list a {} list b {}", list_a, list_b);
        let mut list_c = LinkedList::<i32>::merge(list_a, list_b);
        println!("merged List is {}", list_c);
        for i in 0..target_vec.len() {
            assert_eq!(target_vec[i], *list_c.get(i as i32).unwrap());
        }
    }
}
