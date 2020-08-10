package com.mcx;

/**
 * Created by mcx on 2020-08-10.
 *
 * 反转链表 II
 * 反转从位置 m 到 n 的链表。请使用一趟扫描完成反转。
 * 说明:
 * 1 ≤ m ≤ n ≤ 链表长度。
 *
 * 示例
 * 输入: 1->2->3->4->5, m = 2, n = 4
 * 输出: 1->4->3->2->5
 */
public class ReverseLinkedList2 {

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

    public static ListNode reverseBetween(ListNode head, int m, int n) {
        ListNode beforeRevNode = null;
        ListNode current = head;

        int i = 1;
        while (current != null) {
            if (i == m - 1) {
                // 记录开始翻转之前的节点
                beforeRevNode = current;
            }
            // 找到开始翻转的节点
            if (i == m) {
                break;
            }
            current = current.next;
            i++;
        }

        if (current == null) {
            return head;
        }

        ListNode reverseNode = new ListNode(0);
        ListNode temp = current;
        while (current != null && i <= n) {
            ListNode next = current.next;
            current.next = reverseNode.next;
            reverseNode.next = current;

            current = next;
            i++;
        }
        // 将翻转后的开始节点的下一个节点指向翻转尾节点的下一个节点
        temp.next = current;

        // 表示头节点没有翻转
        if (beforeRevNode != null) {
            // 将翻转后的链表拼接在翻转之前的节点后面
            beforeRevNode.next = reverseNode.next;
            return head;
        } else { // 头节点翻转
            return reverseNode.next;
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

        int m = 2;
        int n = 4;

        printListNode(head);

        System.out.printf("m=%d, ", m);
        System.out.printf("n=%d\n", n);

        head = reverseBetween(head, m, n);

        printListNode(head);
    }
}
