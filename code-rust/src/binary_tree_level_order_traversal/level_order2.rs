/// 二叉树的层次遍历 II
/// 给定一个二叉树，返回其节点值自底向上的层次遍历。 （即按从叶子节点所在层到根节点所在的层，逐层从左向右遍历）

/// 示例
/// 输入: [3,9,20,null,null,15,7]
///        3
///       / \
///      9  20
///        /  \
///       15   7
/// 输出: [
///        [15,7],
///        [9,20],
///        [3]
///      ]

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
    pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if root.is_none() {
            return Vec::new();
        }
        // 当做队列
        let mut q = Vec::new();
        // 根节点先入队列
        q.push(root);
        let mut results = Vec::new();
        while q.len() > 0 {
            let mut val = Vec::new();
            // 临时存放下一级节点
            let mut p = Vec::new();
            // 遍历当前层级的所有节点
            for i in 0..q.len() {
                // 当前层级的根节点
                let node = q.get(i).unwrap();
                val.push(node.as_ref().unwrap().borrow().val);
                if node.as_ref().unwrap().borrow().left.is_some() {
                    p.push(node.as_ref().unwrap().borrow().left.clone());
                }
                if node.as_ref().unwrap().borrow().right.is_some() {
                    p.push(node.as_ref().unwrap().borrow().right.clone());
                }
            }
            // 将当前所有的结果添加到最前面
            results.insert(0, val);

            // 修改成下一次遍历的队列
            q = p;
        }
        results
    }
}

#[test]
fn test_level_order_bottom() {
    let mut root = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    let node1 = Some(Rc::new(RefCell::new(TreeNode::new(9))));
    let mut node2 = Some(Rc::new(RefCell::new(TreeNode::new(20))));
    let node3 = Some(Rc::new(RefCell::new(TreeNode::new(15))));
    let node4 = Some(Rc::new(RefCell::new(TreeNode::new(7))));

    node2.as_mut().unwrap().borrow_mut().left = node3;
    node2.as_mut().unwrap().borrow_mut().right = node4;
    root.as_mut().unwrap().borrow_mut().left = node1;
    root.as_mut().unwrap().borrow_mut().right = node2;

    let results = Solution::level_order_bottom(root);
    println!("{:?}", results);
}
