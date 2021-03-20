/// 不同的二叉搜索树 II
/// 给定一个整数 n，生成所有由 1 ... n 为节点所组成的 二叉搜索树 。

/// 示例
/// 输入: 3
/// 输出:
/// [
///  [1,null,3,2],
///  [3,2,null,1],
///  [3,1,null,null,2],
///  [2,1,3],
///  [1,null,2,null,3]
/// ]
/// 解释：
/// 以上的输出对应以下 5 种不同结构的二叉搜索树：
///   1         3     3      2      1
///    \       /     /      / \      \
///     3     2     1      1   3      2
///    /     /       \                 \
///   2     1         2                 3

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
    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if n == 0 {
            return Vec::new();
        }
        Solution::generate_sub_tree(1, n as usize)
    }

    fn generate_sub_tree(start: usize, end: usize) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if start > end {
            return vec![None];
        }
        let mut tree_nodes = Vec::new();
        for i in start..=end {
            // 获取所有可能的左子节点，由于递归左子树已经生成
            let left_nodes = Solution::generate_sub_tree(start, i - 1);
            // 获取所有可能的右子节点，由于递归右子树已经生成
            let right_nodes = Solution::generate_sub_tree(i + 1, end);
            for left_node in left_nodes {
                for right_node in right_nodes.iter() {
                    let root = Rc::new(RefCell::new(TreeNode::new(i as i32)));
                    // 选取一个节点作为左子节点
                    root.borrow_mut().left = left_node.clone();
                    // 选取一个节点作为右子节点
                    root.borrow_mut().right = right_node.clone();
                    tree_nodes.push(Some(root));
                }
            }
        }
        tree_nodes
    }

    fn pre_order(node: Option<Rc<RefCell<TreeNode>>>) {
        if node.is_none() {
            return;
        }
        print!("{:2}", node.as_ref().unwrap().borrow().val);
        Solution::pre_order(node.as_ref().unwrap().borrow().left.clone());
        Solution::pre_order(node.as_ref().unwrap().borrow().right.clone());
    }
}

#[test]
fn test_generate_trees() {
    let n = 3;

    println!("{}", n);

    let trees = Solution::generate_trees(n);

    for tree in trees.into_iter() {
        Solution::pre_order(tree);
        println!();
    }
}
