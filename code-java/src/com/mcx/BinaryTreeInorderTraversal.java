package com.mcx;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.Stack;

/**
 * Created by mcx on 2020-09-12.
 *
 * 二叉树的中序遍历
 * 给定一个二叉树，返回它的中序遍历。
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
 * 输出: [1,3,2]
 */
public class BinaryTreeInorderTraversal {

    public static class TreeNode {
        int val;
        TreeNode left;
        TreeNode right;
        TreeNode(int val) { this.val = val; }
    }

    public static List<Integer> inorderTraversal(TreeNode root) {
        // 非递归
        return inorderNoRecursion(root);
        // 递归
        /*List<Integer> output = new ArrayList<>();
        inorderRecursion(root, output);
        return output;*/
    }

    /**
     * 非递归中序遍历
     */
    public static List<Integer> inorderNoRecursion(TreeNode node) {
        Stack<TreeNode> stack = new Stack<>();
        List<Integer> output = new ArrayList<>();
        // 将当前节点压入栈
        stack.push(node);
        // 记录每次迭代时的节点
        TreeNode current = node;
        while (!stack.isEmpty()) {
            // 将当前节点的所有最左边的节点入栈
            while (current.left != null) {
                // 将最左边的节点压入栈
                stack.push(current.left);
                current = current.left;
            }

            // 弹出当前节点
            current = stack.pop();
            output.add(current.val);

            // 当前节点的右子节点不为空, 重复循环
            if (current.right != null) {
                // 将右子节点压入栈
                stack.push(current.right);
                current = current.right;
            } else {
                // 当前节点的右子节点为空, 赋值为一个新的节点, 避免继续重复将最左边的节点压入栈
                current = new TreeNode(-1);
            }
        }

        return output;
    }

    /**
     * 递归中序遍历
     */
    public static void inorderRecursion(TreeNode node, List<Integer> output) {
        if (node == null) {
            return;
        }
        // 先访问左子节点
        inorderRecursion(node.left, output);
        // 再将当前节点存入集合
        output.add(node.val);
        // 最后访问右子节点
        inorderRecursion(node.right, output);
    }

    public static void main(String[] args) {
        TreeNode node1 = new TreeNode(1);
        TreeNode node2 = new TreeNode(2);
        TreeNode node3 = new TreeNode(3);
        node1.right = node2;
        node2.left = node3;

        List<Integer> results = inorderTraversal(node1);

        System.out.println(Arrays.toString(results.toArray()));
    }
}
