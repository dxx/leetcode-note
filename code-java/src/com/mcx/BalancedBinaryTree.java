package com.mcx;

/**
 * Created by mcx on 2020-08-18.
 *
 * 平衡二叉树
 * 给定一个二叉树，判断它是否是高度平衡的二叉树。
 * 一棵高度平衡二叉树定义为：一个二叉树每个节点 的左右两个子树的高度差的绝对值不超过 1。
 *
 * 示例
 * 输入: [3,9,20,null,null,15,7]
 *     3
 *    / \
 *   9  20
 *     /  \
 *    15   7
 * 输出: true
 *
 * 输入: [1,2,2,3,3,null,null,4,4]
 *        1
 *       / \
 *      2   2
 *     / \
 *    3   3
 *   / \
 *  4   4
 * 输出: false
 */
public class BalancedBinaryTree {

    public static class TreeNode {
        int val;
        TreeNode left;
        TreeNode right;
        TreeNode(int val) { this.val = val; }
    }

    public static boolean isBalanced(TreeNode root) {
        if (root == null) {
            return true;
        }
        // 判断当前节点的左右子树的高度差
        if (Math.abs(height(root.left) - height(root.right)) > 1) {
            return false;
        }
        // 判断左子节点是否平衡
        boolean isTrue = isBalanced(root.left);
        if (!isTrue) {
            return false;
        }
        // 判断右子节点是否平衡
        return isBalanced(root.right);
    }

    public static int height(TreeNode node) {
        if (node == null) {
            return 0;
        }
        // 左子树高度
        int lHeight = height(node.left);
        // 右子树高度
        int rHeight = height(node.right);
        // 返回最大高度, 当前节点的高度为 1
        return Math.max(lHeight, rHeight) + 1;
    }

    public static void infixOrder(TreeNode node) {
        if (node == null) {
            return;
        }
        infixOrder(node.left);
        System.out.println(node.val);
        infixOrder(node.right);
    }

    public static void main(String[] args) {
        TreeNode node1 = new TreeNode(3);
        TreeNode node2 = new TreeNode(9);
        TreeNode node3 = new TreeNode(20);
        TreeNode node4 = new TreeNode(15);
        TreeNode node5 = new TreeNode(7);
        node1.left = node2;
        node1.right = node3;
        node3.left = node4;
        node3.right = node5;

        infixOrder(node1);

        boolean result = isBalanced(node1);

        System.out.println(result);

        node1 = new TreeNode(1);
        node2 = new TreeNode(2);
        node3 = new TreeNode(2);
        node4 = new TreeNode(3);
        node5 = new TreeNode(3);
        TreeNode node6 = new TreeNode(4);
        TreeNode node7 = new TreeNode(4);
        node1.left = node2;
        node1.right = node3;
        node2.left = node4;
        node2.right = node5;
        node4.left = node6;
        node4.right = node7;

        infixOrder(node1);

        result = isBalanced(node1);

        System.out.println(result);
    }
}
