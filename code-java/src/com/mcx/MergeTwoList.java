package com.mcx;

/**
 * Created by mcx on 2020-06-28.
 *
 * 合并两个有序链表
 * 将两个升序链表合并为一个新的 **升序** 链表并返回。新链表是通过拼接给定的两个链表的所有节点组成的。
 *
 * 示例
 * 输入：1->2->4, 1->3->4
 * 输出：1->1->2->3->4->4
 */
public class MergeTwoList {
    /**
     * 单链表
     */
    public static class ListNode {
        public int val;
        public ListNode next;
        public ListNode(int val) {
            this.val = val;
        }
        public ListNode(int val, ListNode next) {
            this.val = val;
            this.next = next;
        }
    }

    public static ListNode mergeTwoLists(ListNode l1, ListNode l2) {
        ListNode newListNode = null;
        ListNode prevNewNode = null;
        ListNode appendNode = null;
        ListNode node1 = l1;
        ListNode node2 = l2;
        while (node1 != null && node2 != null) {
            if (node1.val <= node2.val) { // 判断左边链表节点是否小于等于右边链表节点
                appendNode = node1;
                // 左边当前链表节点后移
                node1 = node1.next;
            } else if (node1.val > node2.val) { // 判断左边链表节点是大于右边链表节点
                appendNode = node2;
                // 右边当前链表节点后移
                node2 = node2.next;
            }
            // 判断是否添加第一个节点
            if (prevNewNode == null) {
                newListNode = appendNode;
            } else {
                // 将下一个节点添加到上一个节点的后面
                prevNewNode.next = appendNode;
            }
            // 记录上一个节点
            prevNewNode = appendNode;
        }

        // 左边链表还有剩余节点，直接添加到末尾
        if (node1 != null) {
            if (prevNewNode == null) {
                newListNode = node1;
            } else {
                prevNewNode.next = node1;
            }
        }

        // 右边链表还有剩余节点，直接添加到末尾
        if (node2 != null) {
            if (prevNewNode == null) {
                newListNode = node2;
            } else {
                prevNewNode.next = node2;
            }
        }

        return newListNode;
    }

    private static void printListNode(ListNode node) {
        StringBuilder sb = new StringBuilder();
        while (node != null) {
            sb.append(node.val + "->");
            node = node.next;
        }
        sb.deleteCharAt(sb.length() - 1);
        sb.deleteCharAt(sb.length() - 1);
        System.out.println(sb.toString());
    }

    public static void main(String[] args) {
        ListNode l1 = new ListNode(1);
        ListNode node1 = new ListNode(2);
        ListNode node2 = new ListNode(4);
        l1.next = node1;
        node1.next = node2;

        ListNode l2 = new ListNode(1);
        ListNode node3 = new ListNode(3);
        ListNode node4 = new ListNode(4);
        l2.next = node3;
        node3.next = node4;

        printListNode(l1);
        printListNode(l2);

        ListNode newListNode = mergeTwoLists(l1, l2);
        printListNode(newListNode);
    }

}
