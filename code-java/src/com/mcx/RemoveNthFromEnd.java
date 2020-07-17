package com.mcx;

/**
 * Created by mcx on 2020-07-17.
 *
 * 删除链表的倒数第N个节点
 * 给定一个链表，删除链表的倒数第 n 个节点，并且返回链表的头结点。
 *
 * 说明:
 * 给定的 n 保证是有效的。
 *
 * 示例
 * 输入: 1->2->3->4->5, n = 2
 * 输出: 1->2->3->5
 */
public class RemoveNthFromEnd {

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

    public static ListNode removeNthFromEnd(ListNode head, int n) {
        if (head == null || n <= 0) {
            return head;
        }

        ListNode prevNode = null;
        // 快指针，表示需要循环多少次
        ListNode fast = head;
        // 慢指针，表示倒数第 n 个节点
        ListNode slow = head;
        while (fast != null) {
            if (n > 0) {
                // 先将快指针走到第 n 个节点
                fast = fast.next;
                n--;
                continue;
            }
            // 记录上一个节点
            prevNode = slow;

            // 快慢指针同时走
            fast = fast.next;
            slow = slow.next;
        }

        // 当第一次 fast 指针全部走完时 prevNode = slow 未执行
        if (prevNode != null) {
            prevNode.next = prevNode.next.next;
        } else {
            // 删除的是第一个节点
            head = head.next;
        }

        return head;
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

        int n = 2;

        printListNode(head);
        System.out.printf("n = %d\n", n);

        head = removeNthFromEnd(head, n);

        printListNode(head);
    }
}
