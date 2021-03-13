package com.dxx.uniquebinarysearchtree;

import java.util.ArrayList;
import java.util.List;

/**
 * 不同的二叉搜索树 II
 * 给定一个整数 n，生成所有由 1 ... n 为节点所组成的 二叉搜索树 。
 *
 * 示例
 * 输入: 3
 * 输出:
 * [
 *  [1,null,3,2],
 *  [3,2,null,1],
 *  [3,1,null,null,2],
 *  [2,1,3],
 *  [1,null,2,null,3]
 * ]
 * 解释：
 * 以上的输出对应以下 5 种不同结构的二叉搜索树：
 *  1         3     3      2      1
 *   \       /     /      / \      \
 *    3     2     1      1   3      2
 *   /     /       \                 \
 *  2     1         2                 3
 */
public class Main {

    public static class TreeNode {
        public int val;
        public TreeNode left;
        public TreeNode right;
        public TreeNode(int val) {
            this.val = val;
        }
    }

    public static void prevOrder(TreeNode node) {
        if (node == null) {
            return;
        }
        System.out.printf("%2d", node.val);
        prevOrder(node.left);
        prevOrder(node.right);
    }

    public static List<TreeNode> generateTrees(int n) {
        if (n == 0) {
            return null;
        }
        return generateSubTrees(1, n);
    }

    public static List<TreeNode> generateSubTrees(int start, int end) {
        List<TreeNode> treeNodes = new ArrayList<>();
        if (start > end) {
            // 如果没有节点，返回 null
            treeNodes.add(null);
            return treeNodes;
        }
        for (int i = start; i <= end; i++) {
            // 获取所有可能的左子节点，由于递归左子树已经生成
            List<TreeNode> leftNodes = generateSubTrees(start, i - 1);
            // 获取所有可能的右子节点，由于递归右子树已经生成
            List<TreeNode> rightNodes = generateSubTrees(i + 1, end);
            for (TreeNode leftNode : leftNodes) {
                for (TreeNode rightNode : rightNodes) {
                    TreeNode root = new TreeNode(i);
                    // 选取一个节点作为左子节点
                    root.left = leftNode;
                    // 选取一个节点作为右子节点
                    root.right = rightNode;
                    treeNodes.add(root);
                }
            }
        }
        return treeNodes;
    }

    public static void main(String[] args) {
        int n = 3;
        System.out.println(n);

        List<TreeNode> trees = generateTrees(n);
        for (TreeNode tree : trees) {
            prevOrder(tree);
            System.out.println();
        }
    }
}
