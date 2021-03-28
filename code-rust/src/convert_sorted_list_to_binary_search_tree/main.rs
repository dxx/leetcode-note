/// 有序链表转换二叉搜索树
/// 给定一个单链表，其中的元素按升序排序，将其转换为高度平衡的二叉搜索树。
/// 本题中，一个高度平衡二叉树是指一个二叉树每个节点 的左右两个子树的高度差的绝对值不超过 1。

/// 示例
/// 输入: [-10, -3, 0, 5, 9]
/// 输出:      0
///          / \
///        -3   9
///        /   /
///      -10  5
///      [0, -3, 9, -10, null, 5]

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

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
    pub fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        let length = Solution::get_length(head.as_ref());
        let (_, tree) = Solution::build_tree(head, 0, length - 1);
        tree
    }
    /// 分治 + 中序遍历构建二叉树
    fn build_tree(
        current: Option<Box<ListNode>>, start: i32, end: i32)
        -> (Option<Box<ListNode>>, Option<Rc<RefCell<TreeNode>>>) {
        if start > end {
            return (current, None)
        }
        // 中间下标，用来分解
        // (start + end) >> 1 也可以是一个中间下标
        let mid = (start + end + 1) >> 1;
        // 构建左子树
        let (current, left) = Solution::build_tree(current, start, mid - 1);
        if let Some(current) = current {
            // 中序遍历，等同于链表遍历
            let mut root = TreeNode::new(current.val);
            root.left = left;
            // 构建右子树
            let (current, right) = Solution::build_tree(current.next, mid + 1, end);
            root.right = right;
            return (current, Some(Rc::new(RefCell::new(root))));
        }
        (current, None)
    }

    fn get_length(mut head: Option<&Box<ListNode>>) -> i32 {
        let mut len = 0;
        while let Some(node) = head {
            head = node.next.as_ref();
            len += 1;
        }
        len
    }
}

#[test]
fn test_sorted_list_to_bst() {
    let mut node1 = Some(Box::new(ListNode::new(-10)));
    let mut node2 = Some(Box::new(ListNode::new(-3)));
    let mut node3 = Some(Box::new(ListNode::new(0)));
    let mut node4 = Some(Box::new(ListNode::new(5)));
    let node5 = Some(Box::new(ListNode::new(9)));

    node4.as_mut().unwrap().next = node5;
    node3.as_mut().unwrap().next = node4;
    node2.as_mut().unwrap().next = node3;
    node1.as_mut().unwrap().next = node2;

    let tree = Solution::sorted_list_to_bst(node1);

    tree.as_ref().unwrap().borrow().infix_order();
}
