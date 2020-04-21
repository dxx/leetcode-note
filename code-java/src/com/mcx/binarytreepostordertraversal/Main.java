package com.mcx.binarytreepostordertraversal;

import com.mcx.BinaryTreePostorderTraversal;

import java.util.LinkedList;
import java.util.List;
import java.util.Stack;

/**
 * 二叉树的后序遍历
 * 给定一个二叉树，返回它的后序遍历。
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
 * 输出: [3,2,1]
 */
public class Main {
    
    public static class TreeNode {
        int val;
        TreeNode left;
        TreeNode right;
        TreeNode(int val) { this.val = val; }
    }

    public static List<Integer> postorderTraversal(TreeNode root) {
        // 非递归
        return postorderNoRecursion(root);
        // 递归
        /*List<Integer> output = new ArrayList<>();
        postorderRecursion(root, output);
        return output;*/
    }

    /**
     * 非递归后序遍历
     */
    public static List<Integer> postorderNoRecursion(TreeNode node) {
        Stack<TreeNode> stack = new Stack<>();
        LinkedList<Integer> output = new LinkedList<>();
        // 将当前节点压入栈
        stack.add(node);
        while (!stack.isEmpty()) {
            // 弹出当前节点
            TreeNode treeNode = stack.pop();
            // 后访问的节点添加到最前面
            output.addFirst(treeNode.val);

            if (treeNode.left != null) {
                // 先将左子节点压入栈, 先入栈的后出栈
                stack.push(treeNode.left);
            }
            if (treeNode.right != null) {
                // 将右子节点压入栈, 后入栈的先出栈
                stack.push(treeNode.right);
            }
        }
        return output;
    }

    /**
     * 递归后序遍历
     */
    public static void postorderRecursion(TreeNode node, List<Integer> output) {
        if (node == null) {
            return;
        }
        // 先访问左子节点
        postorderRecursion(node.left, output);
        // 再访问右子节点
        postorderRecursion(node.right, output);
        // 最后将当前节点存入数组
        output.add(node.val);
    }

    public static void main(String[] args) {
        TreeNode node1 = new TreeNode(1);
        TreeNode node2 = new TreeNode(2);
        TreeNode node3 = new TreeNode(3);
        node1.right = node2;
        node2.left = node3;

        List<Integer> results = postorderTraversal(node1);

        System.out.println(results);
    }
}
