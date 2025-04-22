/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/

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
    
    // 在节点中插入一个值
    fn insert(&mut self, value: T) {
        // 比较要插入的值与当前节点的值
        match value.cmp(&self.value) {
            // 如果要插入的值小于当前节点的值，则插入到左子树
            Ordering::Less => {
                match self.left {
                    // 如果左子树存在，递归插入
                    Some(ref mut left) => left.insert(value),
                    // 如果左子树不存在，创建一个新节点
                    None => self.left = Some(Box::new(TreeNode::new(value))),
                }
            },
            // 如果要插入的值大于当前节点的值，则插入到右子树
            Ordering::Greater => {
                match self.right {
                    // 如果右子树存在，递归插入
                    Some(ref mut right) => right.insert(value),
                    // 如果右子树不存在，创建一个新节点
                    None => self.right = Some(Box::new(TreeNode::new(value))),
                }
            },
            // 如果要插入的值等于当前节点的值，不做任何操作（避免重复）
            Ordering::Equal => {},
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

    // 在树中插入一个值
    fn insert(&mut self, value: T) {
        match self.root {
            // 如果根节点存在，递归插入
            Some(ref mut root) => root.insert(value),
            // 如果根节点不存在，创建一个新节点作为根节点
            None => self.root = Some(Box::new(TreeNode::new(value))),
        }
    }

    // 在树中搜索一个值
    fn search(&self, value: T) -> bool {
        // 辅助函数，递归搜索
        fn search_in_node<T: Ord>(node: &Option<Box<TreeNode<T>>>, value: &T) -> bool {
            match node {
                // 如果节点不存在，返回 false
                None => false,
                // 如果节点存在，比较值
                Some(boxed_node) => {
                    match value.cmp(&boxed_node.value) {
                        // 如果要搜索的值小于当前节点的值，在左子树中搜索
                        Ordering::Less => search_in_node(&boxed_node.left, value),
                        // 如果要搜索的值大于当前节点的值，在右子树中搜索
                        Ordering::Greater => search_in_node(&boxed_node.right, value),
                        // 如果要搜索的值等于当前节点的值，返回 true
                        Ordering::Equal => true,
                    }
                }
            }
        }
        
        // 从根节点开始搜索
        search_in_node(&self.root, &value)
    }
}