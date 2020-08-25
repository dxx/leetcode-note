package com.mcx;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.LinkedList;
import java.util.List;

/**
 * Created by mcx on 2020-08-25.
 *
 * 二叉树的层序遍历
 * 给你一个二叉树，请你返回其按 **层序遍历** 得到的节点值。 （即逐层地，从左到右访问所有节点）。
 *
 * 示例
 * 输入: [3,9,20,null,null,15,7]
 *        3
 *       / \
 *      9  20
 *        /  \
 *       15   7
 * 输出: [
 *        [3],
 *        [9,20],
 *        [15,7]
 *      ]
 */
public class BinaryTreeLevelOrderTraversal {

    public static class TreeNode {
        int val;
        TreeNode left;
        TreeNode right;
        TreeNode(int val) { this.val = val; }
    }

    public static List<List<Integer>> levelOrder(TreeNode root) {
        return bfs(root);

        /*List<List<Integer>> results = new ArrayList<>();
        dfs(root, 1, results);
        return results;*/
    }

    /**
     * 广度优先
     */
    public static List<List<Integer>> bfs(TreeNode root) {
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
            results.add(val);
        }
        return results;
    }

    /**
     * 深度优先
     */
    public static void dfs(TreeNode node, int level, List<List<Integer>> results) {
        if (node == null) {
            return;
        }
        if (level - 1 >= results.size()) {
            List<Integer> val = new ArrayList<>();
            results.add(val);
        }
        results.get(level - 1).add(node.val);
        // 递归左子节点
        dfs(node.left, level + 1, results);
        // 递归右子节点
        dfs(node.right, level + 1, results);
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

        List<List<Integer>> results = levelOrder(root);
        System.out.println("[");
        for (List<Integer> result : results) {
            System.out.println(Arrays.toString(result.toArray()));
        }
        System.out.println("]");
    }
}
