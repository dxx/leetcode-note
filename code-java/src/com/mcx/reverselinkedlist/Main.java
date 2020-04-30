package com.mcx.reverselinkedlist;

/**
 * 反转链表
 * 反转一个单链表。
 *
 * 示例
 * 输入: 1->2->3->4->5
 * 输出: 5->4->3->2->1
 */
public class Main {

    /**
     * 单链表
     */
    public static class ListNode {
        public int val;
        public ListNode next;
        public ListNode(int val) {
            this.val = val;
        }
    }

    public static ListNode reverseList(ListNode head) {
        ListNode reverseNode = new ListNode(0);
        ListNode current = head;
        while (current != null) {
            // 记录下一个节点
            ListNode next = current.next;
            // 将当前节点指向翻转后的第一个节点
            current.next = reverseNode.next;
            // 修改新的头节点的下一个节点
            reverseNode.next = current;

            // 指向下一个节点
            current = next;
        }
        return reverseNode.next;
    }

    private static void printListNode(ListNode node) {
        StringBuilder sb = new StringBuilder();
        while (node != null) {
            sb.append(node.val).append("->");
            node = node.next;
        }
        sb.deleteCharAt(sb.length() - 1);
        sb.deleteCharAt(sb.length() - 1);
        System.out.println(sb.toString());
    }

    public static void main(String[] args) {
        ListNode head = new ListNode(1);
        ListNode node2 = new ListNode(2);
        ListNode node3 = new ListNode(3);
        ListNode node4 = new ListNode(4);
        ListNode node5 = new ListNode(5);
        head.next = node2;
        node2.next = node3;
        node3.next = node4;
        node4.next = node5;

        printListNode(head);

        head = reverseList(head);

        printListNode(head);
    }
}
