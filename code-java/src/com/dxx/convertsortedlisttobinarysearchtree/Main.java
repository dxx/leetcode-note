package com.dxx.convertsortedlisttobinarysearchtree;

/**
 * 有序链表转换二叉搜索树
 * 给定一个单链表，其中的元素按升序排序，将其转换为高度平衡的二叉搜索树。
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

    public static class ListNode {
        public int val;
        public ListNode next;
        public ListNode(int val) { this.val = val; }
    }

    public static class TreeNode {
        int val;
        TreeNode left;
        TreeNode right;
        TreeNode(int val) { this.val = val; }
    }

    public static TreeNode sortedListToBST(ListNode head) {
        // return buildTree(head, null);
        gCurrent = head;
        int length = getLength(head);
        return buildTree2(0, length - 1);
    }

    /**
     * 构建平衡二叉树
     */
    public static TreeNode buildTree(ListNode head, ListNode tail) {
        if (head == tail) {
            return null;
        }
        ListNode middleNode = getMiddleNode(head, tail);
        // 链表的中间节点作为树的根节点
        TreeNode root = new TreeNode(middleNode.val);
        // 构建左子树
        root.left = buildTree(head, middleNode);
        // 构建右子树
        root.right = buildTree(middleNode.next, tail);
        return root;
    }

    /**
     * 获取中间节点
     */
    public static ListNode getMiddleNode(ListNode head, ListNode tail) {
        ListNode fast = head, slow = head;
        while (fast != tail && fast.next != tail) {
            // 快指针走两个节点
            fast = fast.next;
            fast = fast.next;
            // 慢指针走一个节点
            slow = slow.next;
        }
        return slow;
    }

    public static int getLength(ListNode head) {
        int l = 0;
        ListNode current = head;
        while (current != null) {
            current = current.next;
            l++;
        }
        return l;
    }

    public static ListNode gCurrent = null;

    /**
     * 分治+中序遍历构建二叉树
     */
    public static TreeNode buildTree2(int start, int end) {
        if (start > end) {
            return null;
        }
        TreeNode root = new TreeNode(0);
        // 中间下标，用来分解
        // (start + end) >> 1 也可以是一个中间下标
        int mid = (start + end) >> 1;
        // 构建左子树
        root.left = buildTree2(start, mid - 1);
        // 中序遍历，等同于链表遍历
        root.val = gCurrent.val;
        gCurrent = gCurrent.next;
        // 构建右子树
        root.right = buildTree2(mid + 1, end);
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
        ListNode node1 = new ListNode(-10);
        ListNode node2 = new ListNode(-3);
        ListNode node3 = new ListNode(0);
        ListNode node4 = new ListNode(5);
        ListNode node5 = new ListNode(9);
        node1.next = node2;
        node2.next = node3;
        node3.next = node4;
        node4.next = node5;

        TreeNode tree = sortedListToBST(node1);

        infixOrder(tree);
    }
}
