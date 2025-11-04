/*
    binary_search tree
    This problem requires you to implement a basic interface for a binary tree
*/

//二叉搜索树的核心规则是：左子树所有节点值 <当前节点值，
//右子树所有节点值> 当前节点值（本题处理重复值时不插入，保持树的唯一性）。

use std::cmp::Ordering;
use std::fmt::Debug;

#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}


#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}


impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }

    // 节点级插入：递归处理子树插入逻辑
    fn insert(&mut self, value: T) {
        match self.value.cmp(&value) {
            // 1. 待插入值 < 当前节点值：插入左子树
            Ordering::Greater => {
                if let Some(left_child) = &mut self.left {
                    left_child.insert(value); // 左子树存在，递归插入左子树
                } else {
                    self.left = Some(Box::new(TreeNode::new(value))); // 左子树不存在，创建新节点作为左子树
                }
            }
            // 2. 待插入值 > 当前节点值：插入右子树
            Ordering::Less => {
                if let Some(right_child) = &mut self.right {
                    right_child.insert(value); // 右子树存在，递归插入右子树
                } else {
                    self.right = Some(Box::new(TreeNode::new(value))); // 右子树不存在，创建新节点作为右子树
                }
            }
            // 3. 待插入值 == 当前节点值：重复值，不插入（符合测试用例要求）
            Ordering::Equal => return,
        }
    }

    // 节点级查找：递归处理子树查找逻辑
    fn search(&self, value: T) -> bool {
        match self.value.cmp(&value) {
            // 1. 目标值 < 当前节点值：去左子树找
            Ordering::Greater => self
                .left
                .as_ref()
                .map_or(false, |left_child| left_child.search(value)),
//.map_or处理 Option 可能为空的情况，为空时返回默认值，非空时对内部值做处理并返回结果，避免写繁琐的 if-let 判断。
//“如果当前节点有左子树，就去左子树里找目标值（返回查找结果）；如果没有左子树，就直接返回 false（表示没找到）。”

/*
map_or 说明：self.left.as_ref().map_or(false, |left| left.search(value)) 是 Rust 简化写法，等价于：

if let Some(left_child) = &self.left {
    left_child.search(value)
} else {
    false
} */
            // 2. 目标值 > 当前节点值：去右子树找
            Ordering::Less => self
                .right
                .as_ref()
                .map_or(false, |right_child| right_child.search(value)),
            // 3. 目标值 == 当前节点值：找到，返回true
            Ordering::Equal => true,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{
    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // 树级插入：处理根节点为空的情况，再调用节点级插入
    //处理 “根节点为空” 的特殊情况：若树为空，直接创建根节点；
//若根节点存在，调用根节点的 insert 方法，递归插入子树。
    fn insert(&mut self, value: T) {
        if let Some(root_node) = &mut self.root {
            root_node.insert(value); // 根节点存在，递归插入子树
        } else {
            self.root = Some(Box::new(TreeNode::new(value))); // 根节点为空，创建根节点
        }
    }

    // 树级查找：处理根节点为空的情况，再调用节点级查找
    //处理 “根节点为空” 的特殊情况：空树直接返回 false；
//若根节点存在，调用根节点的 search 方法，递归查找子树。
    fn search(&self, value: T) -> bool {
        self.root.as_ref().map_or(false, |root_node| root_node.search(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        assert_eq!(bst.search(1), false);

        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        bst.insert(1);
        bst.insert(1);

        assert_eq!(bst.search(1), true);

        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            }
            None => panic!("Root should not be None after insertion"),
        }
    }
}
