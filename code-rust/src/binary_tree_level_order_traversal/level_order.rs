/// 二叉树的层序遍历
/// 给你一个二叉树，请你返回其按 **层序遍历** 得到的节点值。 （即逐层地，从左到右访问所有节点）。

/// 示例
/// 输入: [3,9,20,null,null,15,7]
///        3
///       / \
///      9  20
///        /  \
///       15   7
/// 输出: [
///        [3],
///        [9,20],
///        [15,7]
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
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        Solution::bfs(root)

        // let mut results = Vec::new();
        // Solution::dfs(root, 1, results)
    }

    /// 广度优先
    fn bfs(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut results = vec![];
        if root.is_none() {
            return results;
        }
        // 当做队列
        let mut q = Vec::new();
        // 根节点先入队列
        q.push(root);
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
            results.push(val);
            // 修改成下一次遍历的队列
            q = p;
        }
        results
    }

    #[allow(dead_code)]
    /// 深度优先
    fn dfs(node: Option<Rc<RefCell<TreeNode>>>, level: i32, mut results: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if node.is_none() {
            return Vec::new()
        }
        if level - 1 >= results.len() as i32 {
            results.push(Vec::new());
        }
        // results 本身的引用会被修改
        results[(level - 1) as usize].push(node.as_ref().unwrap().borrow().val);

        // 递归左子节点
        let left_results = Solution::dfs(node.as_ref().unwrap().borrow().left.clone(),
                                         level + 1, results.clone());
        if left_results.len() > 0 {
            // 重新修改结果集
            results = left_results;
        }
        // 递归右子节点
        let right_results = Solution::dfs(node.as_ref().unwrap().borrow().right.clone(),
                                         level + 1, results.clone());
        if right_results.len() > 0 {
            results = right_results;
        }
        results
    }
}

#[test]
fn test_level_order() {
    let mut root = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    let node1 = Some(Rc::new(RefCell::new(TreeNode::new(9))));
    let mut node2 = Some(Rc::new(RefCell::new(TreeNode::new(20))));
    let node3 = Some(Rc::new(RefCell::new(TreeNode::new(15))));
    let node4 = Some(Rc::new(RefCell::new(TreeNode::new(7))));

    node2.as_mut().unwrap().borrow_mut().left = node3;
    node2.as_mut().unwrap().borrow_mut().right = node4;
    root.as_mut().unwrap().borrow_mut().left = node1;
    root.as_mut().unwrap().borrow_mut().right = node2;

    let results = Solution::level_order(root);
    println!("{:?}", results);
}
