package com.mcx;

/**
 * Created by mcx on 2020-08-07.
 *
 * 相同的树
 * 给定两个二叉树，编写一个函数来检验它们是否相同。
 * 如果两个树在结构上相同，并且节点具有相同的值，则认为它们是相同的。
 *
 * 示例
 * 输入:   1         1
 *       / \       / \
 *      2   3     2   3
 *
 *      [1,2,3],   [1,2,3]
 *
 * 输出: true
 *
 * 输入:    1          1
 *        /            \
 *       2              2
 *
 *      [1,2],     [1,null,2]
 *
 * 输出: false
 *
 * 输入:    1         1
 *        / \       / \
 *       2   1     1   2
 *
 *      [1,2,1],   [1,1,2]
 *
 * 输出: false
 */
public class SameTree {

    public static class TreeNode {
        int val;
        TreeNode left;
        TreeNode right;
        TreeNode(int val) { this.val = val; }
    }

    public static boolean isSameTree(TreeNode p, TreeNode q) {
        if (p == null && q == null) {
            return true;
        }
        if (p == null || q == null) {
            return false;
        }
        // 比较左子树
        boolean isSame = isSameTree(p.left, q.left);
        // 不相等直接返回
        if (!isSame) {
            return false;
        }
        // 中序比较当前节点值是否相等
        if (p.val != q.val) {
            return false;
        }
        // 比较右子树
        return isSameTree(p.right, q.right);
    }

    public static void infixOrder(TreeNode node) {
        if (node == null) {
            return;
        }
        infixOrder(node.left);
        System.out.print(node.val);
        infixOrder(node.right);
    }

    public static void main(String[] args) {
        TreeNode p = new TreeNode(1);
        TreeNode p2 = new TreeNode(2);
        TreeNode p3 = new TreeNode(3);
        p.left = p2;
        p.right = p3;

        TreeNode q = new TreeNode(1);
        TreeNode q2 = new TreeNode(2);
        TreeNode q3 = new TreeNode(3);
        q.left = q2;
        q.right = q3;

        System.out.print("p: ");
        infixOrder(p);
        System.out.println();

        System.out.print("q: ");
        infixOrder(q);
        System.out.println();

        boolean isSame = isSameTree(p, q);
        System.out.println(isSame);


        p = new TreeNode(1);
        p2 = new TreeNode(2);
        p.left = p2;

        q = new TreeNode(1);
        q2 = new TreeNode(2);
        q.right = q2;

        System.out.print("p: ");
        infixOrder(p);
        System.out.println();

        System.out.print("q: ");
        infixOrder(q);
        System.out.println();

        isSame = isSameTree(p, q);
        System.out.println(isSame);


        p = new TreeNode(1);
        p2 = new TreeNode(2);
        p3 = new TreeNode(1);
        p.left = p2;
        p.right = p3;

        q = new TreeNode(1);
        q2 = new TreeNode(1);
        q3 = new TreeNode(2);
        q.left = q2;
        q.right = q3;

        System.out.print("p: ");
        infixOrder(p);
        System.out.println();

        System.out.print("q: ");
        infixOrder(q);
        System.out.println();

        isSame = isSameTree(p, q);
        System.out.println(isSame);
    }
}
