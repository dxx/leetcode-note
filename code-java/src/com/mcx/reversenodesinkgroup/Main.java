package com.mcx.reversenodesinkgroup;

/**
 * K 个一组翻转链表
 * 给你一个链表，每 k 个节点一组进行翻转，请你返回翻转后的链表。
 * k 是一个正整数，它的值小于或等于链表的长度。
 * 如果节点总数不是 k 的整数倍，那么请将最后剩余的节点保持原有顺序。
 *
 * 示例
 * 输入: 1->2->3->4->5
 * 当 k = 2 时, 返回: 2->1->4->3->5
 * 当 k = 3 时, 返回: 3->2->1->4->5
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

    public static ListNode reverseKGroup(ListNode head, int k) {
        ListNode tail = head;
        // 寻找从 head 节点开始的第 k 个节点的下一个节点
        for (int i = 0; i < k; i++) {
            // 不足 k 个节点不翻转
            if (tail == null) {
                return head;
            }
            tail = tail.next;
        }
        // 翻转 head 节点，不翻转 tail
        ListNode nextHead = reverse(head, tail);
        // 递归翻转，连接新的头结点
        head.next = reverseKGroup(tail, k);
        return nextHead;
    }

    /**
     * 给定起始节点和尾节点翻转链表，返回新的起始节点（不翻转尾节点）
     */
    private static ListNode reverse(ListNode head, ListNode tail) {
        ListNode temp = head;
        ListNode last = null;
        while (temp != tail) {
            // 下一个节点
            ListNode next = temp.next;
            // 当前节点的下一个节点指向尾节点
            temp.next = last;
            // 记录下一个节点
            last = temp;
            temp = next;
        }
        return last;
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

        int k = 2;

        System.out.printf("k=%d时, ", k);

        head = reverseKGroup(head, k);

        printListNode(head);

        head = new ListNode(1);
        node2 = new ListNode(2);
        node3 = new ListNode(3);
        node4 = new ListNode(4);
        node5 = new ListNode(5);
        head.next = node2;
        node2.next = node3;
        node3.next = node4;
        node4.next = node5;

        k = 3;

        System.out.printf("k=%d时, ", k);

        head = reverseKGroup(head, k);

        printListNode(head);
    }
}
