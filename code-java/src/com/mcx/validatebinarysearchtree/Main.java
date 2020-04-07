package com.mcx.validatebinarysearchtree;

/**
 * 验证二叉搜索树
 * 给定一个二叉树，判断其是否是一个有效的二叉搜索树。
 * 假设一个二叉搜索树具有如下特征：
 * 节点的左子树只包含小于当前节点的数。
 * 节点的右子树只包含大于当前节点的数。
 * 所有左子树和右子树自身必须也是二叉搜索树。
 *
 * 示例
 * 输入: [2,1,3]
 *         2
 *        / \
 *       1   3
 * 输出: true
 *
 * 输入: [5,1,4,null,null,3,6]
 *         5
 *        / \
 *       1   4
 *          / \
 *         3   6
 * 输出: false
 * 解释: 根节点的值为 5 ，但是其右子节点值为 4 。
 */
public class Main {

    public static class TreeNode {
        int val;
        TreeNode left;
        TreeNode right;
        TreeNode(int val) { this.val = val; }
    }

    public static boolean isValidBST(TreeNode root) {
        return compare(root, Integer.MIN_VALUE, Integer.MAX_VALUE);
    }

    public static boolean compare(TreeNode node, int lower, int upper) {
        if (node == null) {
            return true;
        }
        // 当前节点小于最小值或者大于最大值
        if (node.val <= lower || node.val >= upper) {
            return false;
        }
        // 左子树区间为最小值-当前节点的值, 右子树区间为当前节点的值-最大值
        return compare(node.left, lower, node.val) && compare(node.right, node.val, upper);
    }

    public static void preOrder(TreeNode node) {
        if (node == null) {
            return;
        }
        System.out.println(node.val);
        preOrder(node.left);
        preOrder(node.right);
    }

    public static void main(String[] args) {
        TreeNode root = new TreeNode(2);
        TreeNode node1 = new TreeNode(1);
        TreeNode node2 = new TreeNode(3);
        root.left = node1;
        root.right = node2;

        preOrder(root);

        boolean isValid = isValidBST(root);
        System.out.println(isValid);

        root = new TreeNode(5);
        node1 = new TreeNode(1);
        node2 = new TreeNode(4);
        TreeNode node3 = new TreeNode(3);
        TreeNode node4 = new TreeNode(6);
        root.left = node1;
        root.right = node2;
        node2.left = node3;
        node2.right = node4;

        preOrder(root);

        isValid = isValidBST(root);
        System.out.println(isValid);
    }
}
