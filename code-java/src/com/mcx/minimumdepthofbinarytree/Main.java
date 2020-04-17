package com.mcx.minimumdepthofbinarytree;

/**
 * 二叉树的最小深度
 * 给定一个二叉树，找出其最小深度。
 * 最小深度是从根节点到最近叶子节点的最短路径上的节点数量。
 * 说明: 叶子节点是指没有子节点的节点。
 *
 * 示例
 * 给定二叉树 [3,9,20,null,null,15,7]
 *   3
 *  / \
 * 9  20
 *   /  \
 *  15   7
 * 它的最小深度是 2 。
 */
public class Main {

    public static class TreeNode {
        int val;
        TreeNode left;
        TreeNode right;
        TreeNode(int x) { val = x; }
    }

    public static int minDepth(TreeNode root) {
        if (root == null) {
            return 0;
        }
        // 左子树的高度
        int lHeight = minDepth(root.left);
        // 右子树的高度
        int rHeight = minDepth(root.right);
        // 左子节点为空，返回右子节点的深度
        if (root.left == null) {
            return rHeight + 1;
        }
        // 右子节点为空，返回左子节点的深度
        if (root.right == null) {
            return lHeight + 1;
        }
        // 返回最小的高度, 当前节点的高度为 1
        return Math.min(lHeight, rHeight) + 1;
    }

    public static void main(String[] args) {
        TreeNode root = new TreeNode(3);
        TreeNode node1 = new TreeNode(9);
        TreeNode node2 = new TreeNode(20);
        TreeNode node3 = new TreeNode(15);
        TreeNode node4 = new TreeNode(7);
        root.left = node1;
        root.right = node2;
        node2.left = node3;
        node2.right = node4;

        int depth = minDepth(root);
        System.out.println(depth);
    }
}
