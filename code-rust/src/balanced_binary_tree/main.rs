/// 平衡二叉树
/// 给定一个二叉树，判断它是否是高度平衡的二叉树。
/// 一棵高度平衡二叉树定义为：一个二叉树每个节点 的左右两个子树的高度差的绝对值不超过 1。

/// 示例
/// 输入: [3,9,20,null,null,15,7]
///     3
///    / \
///   9  20
///     /  \
///    15   7
/// 输出: true

/// 输入: [1,2,2,3,3,null,null,4,4]
///        1
///       / \
///      2   2
///     / \
///    3   3
///   / \
///  4   4
/// 输出: false

#[derive(Debug, PartialEq, Eq, code_rust_macro::InfixOrder)]
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
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root.is_none() {
            return true;
        }
        let root_borrow = root.as_ref().unwrap().borrow();
        // 判断当前节点的左右子树的高度差
        if (Solution::height(root_borrow.left.clone()) -
            Solution::height(root_borrow.right.clone())).abs() > 1 {
            return false;
        }
        // 判断左子节点是否平衡
        let is_true = Solution::is_balanced(root_borrow.left.clone());
        if !is_true {
            return false
        }
        // 判断右子节点是否平衡
        Solution::is_balanced(root_borrow.right.clone())
    }

    fn height(node: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if node.is_none() {
            return 0;
        }
        // 左子树高度
        let l_height = Solution::height(node.as_ref().unwrap().borrow().left.clone());
        // 右子树高度
        let r_height = Solution::height(node.as_ref().unwrap().borrow().right.clone());
        // 返回最大高度, 当前节点的高度为 1
        l_height.max(r_height) + 1
    }

}

#[test]
fn test_is_balanced() {
    let mut node1 = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    let node2 = Some(Rc::new(RefCell::new(TreeNode::new(9))));
    let mut node3 = Some(Rc::new(RefCell::new(TreeNode::new(20))));
    let node4 = Some(Rc::new(RefCell::new(TreeNode::new(15))));
    let node5 = Some(Rc::new(RefCell::new(TreeNode::new(7))));

    node3.as_mut().unwrap().borrow_mut().left = node4;
    node3.as_mut().unwrap().borrow_mut().right = node5;
    node1.as_mut().unwrap().borrow_mut().left = node2;
    node1.as_mut().unwrap().borrow_mut().right = node3;

    node1.as_ref().unwrap().borrow().infix_order();

    let result = Solution::is_balanced(node1);
    println!("{}", result);


    let mut node1 = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    let mut node2 = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    let node3 = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    let mut node4 = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    let node5 = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    let node6 = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    let node7 = Some(Rc::new(RefCell::new(TreeNode::new(4))));

    node4.as_mut().unwrap().borrow_mut().left = node6;
    node4.as_mut().unwrap().borrow_mut().right = node7;
    node2.as_mut().unwrap().borrow_mut().left = node4;
    node2.as_mut().unwrap().borrow_mut().right = node5;
    node1.as_mut().unwrap().borrow_mut().left = node2;
    node1.as_mut().unwrap().borrow_mut().right = node3;

    node1.as_ref().unwrap().borrow().infix_order();

    let result = Solution::is_balanced(node1);
    println!("{}", result);
}
