/// 二叉树的前序遍历
/// 给定一个二叉树，返回它的前序遍历。

/// 进阶: 递归算法很简单，你可以通过迭代算法完成吗？

/// 示例
/// 输入: [1,null,2,3]
///   1
///    \
///     2
///    /
///   3
/// 输出: [1,2,3]

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
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        // 非递归
        Solution::preorder_no_recursion(root)
        // 递归
        // Solution::preorder_recursion(root)
    }

    fn preorder_no_recursion(node: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut stack = Vec::new();
        let mut output = Vec::new();
        if node.is_none() {
            return output;
        }
        // 将当前节点压入栈
        stack.push(node.clone());
        while stack.len() > 0 {
            // 弹出当前节点
            let node = stack.pop().unwrap();
            let node = node.as_ref().unwrap().borrow();
            output.push(node.val);

            if let Some(right) = &node.right {
                // 先将右子节点压入栈, 先入栈的后出栈
                stack.push(Some(right.clone()));
            }
            if let Some(ref left) = &node.left {
                // 将左子节点压入栈, 后入栈的先出栈
                stack.push(Some(left.clone()));
            }
        }
        output
    }

    #[allow(dead_code)]
    /// 递归前序遍历
    fn preorder_recursion(node: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if node.is_none() {
            return Vec::new();
        }
        let root = node.unwrap();
        let mut nodes = Vec::new();
        // 先将当前节点存入向量
        nodes.push(root.borrow().val);
        // 再访问左子节点
        nodes.append(&mut Solution::preorder_recursion(
            root.borrow().left.clone()));
        // 最后访问右子节点
        nodes.append(&mut Solution::preorder_recursion(
            root.borrow().right.clone(),
        ));

        nodes
    }
}

#[test]
fn test_preorder_traversal() {
    let mut node1 = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    let mut node2 = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    let node3 = Some(Rc::new(RefCell::new(TreeNode::new(3))));

    node2.as_mut().unwrap().borrow_mut().left = node3;
    node1.as_mut().unwrap().borrow_mut().right = node2;

    let results = Solution::preorder_traversal(node1);

    println!("{:?}", results);
}
