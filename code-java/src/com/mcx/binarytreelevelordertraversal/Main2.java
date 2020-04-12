package com.mcx.binarytreelevelordertraversal;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.LinkedList;
import java.util.List;

/**
 * 二叉树的层次遍历 II
 * 给定一个二叉树，返回其节点值自底向上的层次遍历。 （即按从叶子节点所在层到根节点所在的层，逐层从左向右遍历）
 *
 * 示例
 * 输入: [3,9,20,null,null,15,7]
 *        3
 *       / \
 *      9  20
 *        /  \
 *       15   7
 * 输出: [
 *        [15,7],
 *        [9,20],
 *        [3]
 *      ]
 */
public class Main2 {

    public static class TreeNode {
        int val;
        TreeNode left;
        TreeNode right;
        TreeNode(int val) { this.val = val; }
    }

    public static List<List<Integer>> levelOrderBottom(TreeNode root) {
        List<List<Integer>> results = new ArrayList<>();
        if (root == null) {
            return results;
        }
        // 当做队列
        LinkedList<TreeNode> p = new LinkedList<>();
        // 根节点先入队列
        p.add(root);
        while (!p.isEmpty()) {
            // 临时存放下一级节点
            List<Integer> val = new ArrayList<>();
            int len = p.size();
            // 遍历当前层级的所有节点
            for (int i = 0; i < len; i++) {
                // 当前层级的根节点
                TreeNode node = p.removeFirst();
                val.add(node.val);
                if (node.left != null) {
                    p.addLast(node.left);
                }
                if (node.right != null) {
                    p.addLast(node.right);
                }
            }
            // 添加到最前面
            results.add(0, val);
        }
        return results;
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

        List<List<Integer>> results = levelOrderBottom(root);
        System.out.println("[");
        for (List<Integer> result : results) {
            System.out.println(Arrays.toString(result.toArray()));
        }
        System.out.println("]");
    }
}
