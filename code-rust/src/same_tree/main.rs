/// 相同的树
/// 给定两个二叉树，编写一个函数来检验它们是否相同。
/// 如果两个树在结构上相同，并且节点具有相同的值，则认为它们是相同的。

/// 示例
/// 输入: [1,2,3],  [1,2,3]
///         1         1
///        / \       / \
///       2   3     2   3
///
/// 输出: true

/// 输入: [1,2],    [1,null,2]
///         1          1
///        /            \
///       2              2
///
/// 输出: false

/// 输入: [1,2,1],  [1,1,2]
///         1         1
///        / \       / \
///       2   1     1   2
///
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
    pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if p.is_none() && q.is_none() {
            return true
        }
        if p .is_none() || q.is_none() {
            return false
        }
        let p_borrow = p.as_ref().unwrap().borrow();
        let q_borrow = q.as_ref().unwrap().borrow();

        // 比较左子树
        let is_same = Solution::is_same_tree(p_borrow.left.clone(), q_borrow.left.clone());
        // 不相等直接返回
        if !is_same {
            return false;
        }
        if p_borrow.val != q_borrow.val {
            return false;
        }
        // 比较右子树
        return Solution::is_same_tree(p_borrow.right.clone(), q_borrow.right.clone());
    }
}

#[test]
fn test_is_same_tree() {
    let mut p = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    let p2 = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    let p3 = Some(Rc::new(RefCell::new(TreeNode::new(3))));

    p.as_mut().unwrap().borrow_mut().left = p2;
    p.as_mut().unwrap().borrow_mut().right = p3;

    let mut q = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    let q2 = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    let q3 = Some(Rc::new(RefCell::new(TreeNode::new(3))));

    q.as_mut().unwrap().borrow_mut().left = q2;
    q.as_mut().unwrap().borrow_mut().right = q3;

    println!("p: ");
    p.as_ref().unwrap().borrow().infix_order();

    println!("q: ");
    q.as_ref().unwrap().borrow().infix_order();

    let is_same = Solution::is_same_tree(p, q);
    println!("{}", is_same);


    let mut p = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    let p2 = Some(Rc::new(RefCell::new(TreeNode::new(2))));

    p.as_mut().unwrap().borrow_mut().left = p2;

    let mut q = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    let q2 = Some(Rc::new(RefCell::new(TreeNode::new(2))));

    q.as_mut().unwrap().borrow_mut().right = q2;

    println!("p: ");
    p.as_ref().unwrap().borrow().infix_order();

    println!("q: ");
    q.as_ref().unwrap().borrow().infix_order();

    let is_same = Solution::is_same_tree(p, q);
    println!("{}", is_same);


    let mut p = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    let p2 = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    let p3 = Some(Rc::new(RefCell::new(TreeNode::new(1))));

    p.as_mut().unwrap().borrow_mut().left = p2;
    p.as_mut().unwrap().borrow_mut().right = p3;

    let mut q = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    let q2 = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    let q3 = Some(Rc::new(RefCell::new(TreeNode::new(2))));

    q.as_mut().unwrap().borrow_mut().left = q2;
    q.as_mut().unwrap().borrow_mut().right = q3;

    println!("p: ");
    p.as_ref().unwrap().borrow().infix_order();

    println!("q: ");
    q.as_ref().unwrap().borrow().infix_order();

    let is_same = Solution::is_same_tree(p, q);
    println!("{}", is_same);
}
