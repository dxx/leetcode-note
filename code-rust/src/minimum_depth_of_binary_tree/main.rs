/// 二叉树的最小深度
/// 给定一个二叉树，找出其最小深度。
/// 最小深度是从根节点到最近叶子节点的最短路径上的节点数量。
/// 说明: 叶子节点是指没有子节点的节点。

/// 示例
/// 给定二叉树 [3,9,20,null,null,15,7]
///   3
///  / \
/// 9  20
///   /  \
///  15   7
/// 它的最小深度是 2 。

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode { val, left: None, right: None }
    }
}

use std::rc::Rc;
use std::cell::RefCell;

pub struct Solution {}

impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        // 左子树的高度
        let l_height = Solution::min_depth(root.as_ref().unwrap().borrow().left.clone());
        // 右子树的高度
        let r_height = Solution::min_depth(root.as_ref().unwrap().borrow().right.clone());
        // 左子节点为空，返回右子节点的深度
        if root.as_ref().unwrap().borrow().left.is_none() {
            return r_height + 1;
        }
        // 右子节点为空，返回左子节点的深度
        if root.as_ref().unwrap().borrow().right.is_none() {
            return l_height + 1;
        }
        // 返回最小的高度, 当前节点的高度为 1
        l_height.min(r_height) + 1
    }
}

#[test]
fn test_min_depth() {
    let mut root = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    let node1 = Some(Rc::new(RefCell::new(TreeNode::new(9))));
    let mut node2 = Some(Rc::new(RefCell::new(TreeNode::new(20))));
    let node3 = Some(Rc::new(RefCell::new(TreeNode::new(15))));
    let node4 = Some(Rc::new(RefCell::new(TreeNode::new(7))));

    node2.as_mut().unwrap().borrow_mut().left = node3;
    node2.as_mut().unwrap().borrow_mut().right = node4;
    root.as_mut().unwrap().borrow_mut().left = node1;
    root.as_mut().unwrap().borrow_mut().right = node2;

    let depth = Solution::min_depth(root);
    println!("{}", depth);
}
