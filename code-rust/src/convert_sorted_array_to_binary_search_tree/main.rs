/// 将有序数组转换为二叉搜索树
/// 将一个按照升序排列的有序数组，转换为一棵高度平衡二叉搜索树。
/// 本题中，一个高度平衡二叉树是指一个二叉树每个节点 的左右两个子树的高度差的绝对值不超过 1。

/// 示例
/// 输入: [-10, -3, 0, 5, 9]
/// 输出:      0
///          / \
///        -3   9
///        /   /
///      -10  5
///      [0, -3, 9, -10, null, 5]

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
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let end = (nums.len() - 1) as i32;
        Solution::build_tree(nums, 0, end)
    }

    fn build_tree(nums: Vec<i32>, start: i32, end: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if start > end {
            return None;
        }
        // (start + end) >> 1 也可以是一个中间下标
        // 选择中间位置的右边一个节点作为根节点
        let mid = (start as usize + end as usize + 1) as i32 >> 1;
        // 数组的中间节点作为树的根节点
        let mut root = TreeNode::new(nums[mid as usize]);
        // 构建左子树
        root.left = Solution::build_tree(nums.clone(), start, mid - 1);
        // 构建右子树
        root.right = Solution::build_tree(nums, mid + 1, end);

        Some(Rc::new(RefCell::new(root)))
    }
}

#[test]
fn test_sorted_array_to_bst() {
    let nums = vec![-10, -3, 0, 5, 9];

    println!("{:?}", nums);

    let tree = Solution::sorted_array_to_bst(nums);

    tree.as_ref().unwrap().borrow().infix_order();
}
