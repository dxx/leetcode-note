/// 对称二叉树
/// 给定一个二叉树，检查它是否是镜像对称的。如果一个树的左子树与右子树结构相同, 对应的节点值也相同，那么这个树是镜像对称的。

/// 示例
/// 输入: [1,2,2,3,4,4,3]
///          1
///         / \
///        2   2
///       / \ / \
///      3  4 4  3
/// 输出: true

/// 输入: [1,2,2,null,3,null,3]
///         1
///        / \
///       2   2
///        \   \
///        3    3
/// 输出: false

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
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Solution::check_symmetrical(root.clone(), root)
    }

    fn check_symmetrical(node1: Option<Rc<RefCell<TreeNode>>>, node2: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if node1.is_none() && node2.is_none() {
            return true
        }
        // 只要有一个节点为 nil, 表示镜像非对称
        if node1.is_none() || node2.is_none() {
            return false
        }
        let node1_borrow = node1.as_ref().unwrap().borrow();
        let node2_borrow = node2.as_ref().unwrap().borrow();
        // 节点不相同表示非镜像对称
        if node1_borrow.val != node2_borrow.val {
            return false
        }
        // 比较两棵树的左右子树是否是镜像对称的
        return Solution::check_symmetrical(node1_borrow.left.clone(), node2_borrow.right.clone()) &&
            Solution::check_symmetrical(node1_borrow.right.clone(), node2_borrow.left.clone());
    }
}

#[test]
fn test_is_symmetric () {
    let mut root = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    let mut node2 = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    let mut node3 = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    let node4 = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    let node5 = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    let node6 = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    let node7 = Some(Rc::new(RefCell::new(TreeNode::new(4))));

    node3.as_mut().unwrap().borrow_mut().left = node7;
    node3.as_mut().unwrap().borrow_mut().right = node5;
    node2.as_mut().unwrap().borrow_mut().left = node4;
    node2.as_mut().unwrap().borrow_mut().right = node6;
    root.as_mut().unwrap().borrow_mut().left = node2;
    root.as_mut().unwrap().borrow_mut().right = node3;

    let result = Solution::is_symmetric(root);

    println!("{}", result);

    let mut root = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    let mut node2 = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    let mut node3 = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    let node4 = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    let node5 = Some(Rc::new(RefCell::new(TreeNode::new(3))));

    node3.as_mut().unwrap().borrow_mut().right = node5;
    node2.as_mut().unwrap().borrow_mut().right = node4;
    root.as_mut().unwrap().borrow_mut().left = node2;
    root.as_mut().unwrap().borrow_mut().right = node3;

    let result = Solution::is_symmetric(root);

    println!("{}", result);
}