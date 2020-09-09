package com.mcx;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.Stack;

/**
 * Created by mcx on 2020-08-16.
 *
 * 二叉树的前序遍历
 * 给定一个二叉树，返回它的前序遍历。
 *
 * 进阶: 递归算法很简单，你可以通过迭代算法完成吗？
 *
 * 示例
 * 输入: [1,null,2,3]
 *   1
 *    \
 *     2
 *    /
 *   3
 * 输出: [1,2,3]
 */
public class BinaryTreePreorderTraversal {

     public static class TreeNode {
         int val;
         TreeNode left;
         TreeNode right;
         TreeNode(int val) { this.val = val; }
     }

    public static List<Integer> preorderTraversal(TreeNode root) {
         // 非递归
         return preorderNoRecursion(root);
         // 递归
         /*List<Integer> output = new ArrayList<>();
         preorderRecursion(root, output);
         return output;*/
    }

    /**
     * 非递归前序遍历
     */
    public static List<Integer> preorderNoRecursion(TreeNode node) {
        Stack<TreeNode> stack = new Stack<>();
        List<Integer> output = new ArrayList<>();
        // 将当前节点压入栈
        stack.add(node);
        while (!stack.isEmpty()) {
            // 弹出当前节点
            TreeNode treeNode = stack.pop();
            output.add(treeNode.val);

            if (treeNode.right != null) {
                // 先将右子节点压入栈, 先入栈的后出栈
                stack.push(treeNode.right);
            }
            if (treeNode.left != null) {
                // 将左子节点压入栈, 后入栈的先出栈
                stack.push(treeNode.left);
            }
        }
        return output;
    }

    /**
     * 递归前序遍历
     */
    public static void preorderRecursion(TreeNode node, List<Integer> output) {
        if (node == null) {
            return;
        }
        // 先将当前节点存入集合
        output.add(node.val);
        // 再访问左子节点
        preorderRecursion(node.left, output);
        // 最后访问右子节点
        preorderRecursion(node.right, output);
    }

    public static void main(String[] args) {
        TreeNode node1 = new TreeNode(1);
        TreeNode node2 = new TreeNode(2);
        TreeNode node3 = new TreeNode(3);
        node1.right = node2;
        node2.left = node3;

        List<Integer> results = preorderTraversal(node1);

        System.out.println(Arrays.toString(results.toArray()));
    }
}
