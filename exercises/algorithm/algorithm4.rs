/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/

//I AM NOT DONE
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
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{

    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        //TODO
        match &mut self.root {
            None => {
                self.root = Some(Box::new(TreeNode::new(value)));
            }
            Some(node) => {
                node.insert(value);
            }
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        //TODO
        match &self.root {
            None => false,  // 空树
            Some(node) => node.search(&value),
        }
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        //TODO
        match value.cmp(&self.value) {
            Ordering::Less => {
                // 插入值小于当前节点值，应该放到左子树
                match &mut self.left {
                    None => {
                        // 创建新节点并存入值
                        self.left = Some(Box::new(TreeNode::new(value)));
                    }
                    Some(left_node) => {
                        // 递归插入
                        left_node.insert(value);
                    }
                }
            }
            Ordering::Greater => {
                // 插入值大于当前节点值，应该放到右子树
                match &mut self.right {
                    None => {
                        // 创建新节点并存入值
                        self.right = Some(Box::new(TreeNode::new(value)));
                    }
                    Some(right_node) => {
                        // 递归插入
                        right_node.insert(value);
                    }
                }
            }
            Ordering::Equal => {

            }
        }
    }
    fn search(&self, value: &T) -> bool {
        match value.cmp(&self.value) {
            Ordering::Less => {
                // 要查找的值小于当前节点值，在左子树中查找
                match &self.left {
                    None => false,  
                    Some(left_node) => left_node.search(value),
                }
            }
            Ordering::Greater => {
                // 要查找的值大于当前节点值，在右子树中查找
                match &self.right {
                    None => false,  
                    Some(right_node) => right_node.search(value),
                }
            }
            Ordering::Equal => true,  // 找到值
        }
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
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}    


