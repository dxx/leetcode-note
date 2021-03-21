/// 验证二叉搜索树
/// 给定一个二叉树，判断其是否是一个有效的二叉搜索树。
/// 假设一个二叉搜索树具有如下特征：
/// 节点的左子树只包含小于当前节点的数。
/// 节点的右子树只包含大于当前节点的数。
/// 所有左子树和右子树自身必须也是二叉搜索树。

/// 示例
/// 输入: [2,1,3]
///         2
///        / \
///       1   3
/// 输出: true

/// 输入: [5,1,4,null,null,3,6]
///         5
///        / \
///       1   4
///         / \
///        3   6
/// 输出: false
/// 解释: 根节点的值为 5 ，但是其右子节点值为 4 。

#[derive(Debug, PartialEq, Eq, code_rust_macro::PreOrder)]
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
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Solution::compare(root, i64::MIN, i64::MAX)
    }

    fn compare(node: Option<Rc<RefCell<TreeNode>>>, lower: i64, upper: i64) -> bool {
        if node.is_none() {
            return true;
        }
        let val = node.as_ref().unwrap().borrow().val as i64;
        // 当前节点小于最小值或者大于最大值
        if val <= lower || val >= upper {
            return false;
        }
        // 左子树区间为最小值-当前节点的值, 右子树区间为当前节点的值-最大值
        Solution::compare(node.as_ref().unwrap().borrow().left.clone(), lower, val) &&
            Solution::compare(node.as_ref().unwrap().borrow().right.clone(), val, upper)
    }
}

#[test]
fn test_is_valid_bst() {
    let mut root = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    let node1 = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    let node2 = Some(Rc::new(RefCell::new(TreeNode::new(3))));

    root.as_mut().unwrap().borrow_mut().left = node1;
    root.as_mut().unwrap().borrow_mut().right = node2;

    root.as_ref().unwrap().borrow().pre_order();

    let is_valid = Solution::is_valid_bst(root);
    println!("{}", is_valid);

    let mut root = Some(Rc::new(RefCell::new(TreeNode::new(5))));
    let node1 = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    let mut node2 = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    let node3 = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    let node4 = Some(Rc::new(RefCell::new(TreeNode::new(6))));

    node2.as_mut().unwrap().borrow_mut().left = node3;
    node2.as_mut().unwrap().borrow_mut().right = node4;
    root.as_mut().unwrap().borrow_mut().left = node1;
    root.as_mut().unwrap().borrow_mut().right = node2;

    root.as_ref().unwrap().borrow().pre_order();

    let is_valid = Solution::is_valid_bst(root);
    println!("{}", is_valid);
}
