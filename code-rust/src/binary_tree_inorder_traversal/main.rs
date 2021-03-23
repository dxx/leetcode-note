/// 二叉树的中序遍历
/// 给定一个二叉树，返回它的中序遍历。

/// 进阶: 递归算法很简单，你可以通过迭代算法完成吗？

/// 示例
/// 输入: [1,null,2,3]
///   1
///    \
///     2
///    /
///   3
/// 输出: [1,3,2]

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode { val, left: None, right: None, }
    }
}

use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution {}

impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        // 非递归
        Solution::inorder_no_recursion(root)
        // 递归
        // Solution::inorder_recursion(root)
    }

    /// 非递归中序遍历
    fn inorder_no_recursion(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut stack = Vec::new();
        let mut output = Vec::new();
        if root.is_none() {
            return output;
        }
        // 将当前节点压入栈
        stack.push(root.clone());
        // 记录每次迭代时的节点
        let mut current = root.clone();
        while stack.len() > 0 {
            // 将当前节点的所有最左边的节点入栈
            while current.as_ref().unwrap().borrow().left.is_some() {
                let left = current.as_ref().unwrap().borrow().left.clone();
                // 将最左边的节点压入栈
                stack.push(left.clone());
                current = left;
            }
            // 弹出当前节点
            current = stack.pop().unwrap();

            output.push(current.as_ref().unwrap().borrow().val);

            // 当前节点的右子节点不为空, 重复循环
            if current.as_ref().unwrap().borrow().right.is_some() {
                let right = current.as_ref().unwrap().borrow().right.clone();
                // 将右子节点压入栈
                stack.push(right.clone());
                current = right;
            } else {
                // 当前节点的右子节点为空, 赋值为一个新的节点, 避免继续重复将最左边的节点压入栈
                current = Some(Rc::new(RefCell::new(TreeNode::new(-1))));
            }
        }
        output
    }

    #[allow(dead_code)]
    /// 递归中序遍历
    fn inorder_recursion(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if root.is_none() {
            return Vec::new();
        }
        let root = root.unwrap();
        let mut nodes = Vec::new();
        // 先访问左子节点
        nodes.append(&mut Solution::inorder_recursion(root.borrow().left.clone()));
        // 再将当前节点存入向量
        nodes.push(root.borrow().val);
        // 最后访问右子节点
        nodes.append(&mut Solution::inorder_recursion(
            root.borrow().right.clone(),
        ));

        nodes
    }
}

#[test]
fn test_inorder_traversal() {
    let mut head = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    let mut node2 = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    let node3 = Some(Rc::new(RefCell::new(TreeNode::new(3))));

    node2.as_mut().unwrap().borrow_mut().left = node3;
    head.as_mut().unwrap().borrow_mut().right = node2;

    let results = Solution::inorder_traversal(head);

    println!("{:?}", results);
}
