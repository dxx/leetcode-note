package com.dxx.convertsortedarraytobinarysearchtree;

import java.util.Arrays;

/**
 * 将有序数组转换为二叉搜索树
 * 将一个按照升序排列的有序数组，转换为一棵高度平衡二叉搜索树。
 * 本题中，一个高度平衡二叉树是指一个二叉树每个节点 的左右两个子树的高度差的绝对值不超过 1。
 *
 * 示例
 * 输入: [-10, -3, 0, 5, 9]
 * 输出:      0
 *          / \
 *        -3   9
 *        /   /
 *      -10  5
 *      [0, -3, 9, -10, null, 5]
 */
public class Main {

    public static class TreeNode {
        int val;
        TreeNode left;
        TreeNode right;
        TreeNode(int val) { this.val = val; }
    }

    public static TreeNode sortedArrayToBST(int[] nums) {
        return buildTree(nums, 0, nums.length - 1);
    }

    public static TreeNode buildTree(int[] nums, int start, int end) {
        if (start > end) {
            return null;
        }
        // (start + end) >> 1 也可以是一个中间下标
        // 选择中间位置的右边一个节点作为根节点
        int mid = (start + end + 1) >> 1;
        // 数组的中间节点作为树的根节点
        TreeNode root = new TreeNode(nums[mid]);
        // 构建左子树
        root.left = buildTree(nums, start, mid - 1);
        // 构建右子树
        root.right = buildTree(nums, mid + 1, end);
        return root;
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
        int[] nums = {-10, -3, 0, 5, 9};

        System.out.println(Arrays.toString(nums));

        TreeNode tree = sortedArrayToBST(nums);

        infixOrder(tree);
    }
}
