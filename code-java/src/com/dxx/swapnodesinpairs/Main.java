package com.dxx.swapnodesinpairs;

/**
 * 两两交换链表中的节点
 * 给定一个链表，两两交换其中相邻的节点，并返回交换后的链表。**你不能只是单纯的改变节点内部的值**，而是需要实际的进行节点交换。
 *
 * 示例
 * 输入: 1->2->3->4
 * 输出: 2->1->4->3
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

    public static ListNode swapPairs(ListNode head) {
        ListNode swapHeadNode = new ListNode(-1); // 新的头结点，用来交换节点
        ListNode lastNode = swapHeadNode; // 保存新节点中之后一个节点
        lastNode.next = head; // 当 head 只有一个节点时，默认就是 head
        ListNode current = head;
        ListNode prevNode, nextNode;
        // 每次遍历两个节点
        while (current != null && current.next != null) {
            prevNode = current; // 前一个节点
            nextNode = current.next; // 后一个节点

            // 保存下一次遍历时的节点
            ListNode next = nextNode.next;

            // 新的链表中最后一个节点指向后一个节点
            lastNode.next = nextNode;

            // 先将上一个节点指向原链表中的下一次遍历时的节点
            prevNode.next = next;

            // 新的链表中最后一个节点指向原链表中的上一个节点
            nextNode.next = prevNode;
            // 记录新的链表中最后一个节点
            lastNode = prevNode;

            current = next;
        }
        return swapHeadNode.next;
    }

    public static void main(String[] args) {
        ListNode head = new ListNode(1);
        ListNode node1 = new ListNode(2);
        ListNode node2 = new ListNode(3);
        ListNode node3 = new ListNode(4);
        head.next = node1;
        node1.next = node2;
        node2.next = node3;

        System.out.print("交换前:");
        printListNode(head);

        ListNode newNode = swapPairs(head);

        System.out.print("交换后:");
        printListNode(newNode);
    }
}
