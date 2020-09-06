package com.mcx;

/**
 * Created by mcx on 2020-09-06.
 *
 * 对称二叉树
 * 给定一个二叉树，检查它是否是镜像对称的。如果一个树的左子树与右子树结构相同, 对应的节点值也相同，那么这个树是镜像对称的。
 *
 * 示例
 * 输入: [1,2,2,3,4,4,3]
 *          1
 *         / \
 *        2   2
 *       / \ / \
 *      3  4 4  3
 * 输出: true
 *
 * 输入: [1,2,2,null,3,null,3]
 *         1
 *        / \
 *       2   2
 *        \   \
 *        3    3
 * 输出: false
 */
public class SymmetricTree {

    public static class TreeNode {
        int val;
        TreeNode left;
        TreeNode right;
        TreeNode(int val) { this.val = val; }
    }

    public static boolean isSymmetric(TreeNode root) {
        return checkSymmetrical(root, root);
    }

    public static boolean checkSymmetrical(TreeNode node1, TreeNode node2) {
        if (node1 == null && node2 == null) {
            return true;
        }
        // 只要有一个节点为 nil, 表示镜像非对称
        if (node1 == null || node2 == null) {
            return false;
        }
        // 节点不相同表示非镜像对称
        if (node1.val != node2.val) {
            return false;
        }
        // 比较两棵树的左右子树是否是镜像对称的
        return checkSymmetrical(node1.left, node2.right) && checkSymmetrical(node1.right, node2.left);
    }

    public static void main(String[] args) {
        TreeNode root = new TreeNode(1);
        TreeNode node2 = new TreeNode(2);
        TreeNode node3 = new TreeNode(2);
        TreeNode node4 = new TreeNode(3);
        TreeNode node5 = new TreeNode(3);
        TreeNode node6 = new TreeNode(4);
        TreeNode node7 = new TreeNode(4);
        root.left = node2;
        root.right = node3;
        node2.left = node4;
        node2.right = node6;
        node3.left = node7;
        node3.right = node5;

        boolean result = isSymmetric(root);

        System.out.println(result);

        root = new TreeNode(1);
        node2 = new TreeNode(2);
        node3 = new TreeNode(2);
        node4 = new TreeNode(3);
        node5 = new TreeNode(3);
        root.left = node2;
        root.right = node3;
        node2.right = node4;
        node3.right = node5;

        result = isSymmetric(root);

        System.out.println(result);
    }
}
