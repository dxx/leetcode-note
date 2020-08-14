package com.mcx;

/**
 * Created by mcx on 2020-08-14.
 *
 * 删除排序链表中的重复元素
 * 给定一个排序链表，删除所有重复的元素，使得每个元素只出现一次。
 *
 * 示例
 * 输入: 1->1->2
 * 输出: 1->2
 *
 * 输入: 1->1->2->3->3
 * 输出: 1->2->3
 */
public class RemoveDuplicatesFromSortedlist {

    public static class ListNode {
        public int val;
        public ListNode next;
        public ListNode(int val) {
            this.val = val;
        }
    }

    public static ListNode deleteDuplicates(ListNode head) {
        ListNode current = head, prevNode = head;
        while (current != null) {
            // 找到下一个不相同的节点
            while (current.next != null && current.val == current.next.val) {
                current = current.next;
            }

            // 指向下一个不相同的节点
            prevNode.next = current.next;

            // 记录上一个节点
            prevNode = current.next;
            // 从下一个不相同的节点继续走
            current = current.next;
        }
        return head;
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
        ListNode node2 = new ListNode(1);
        ListNode node3 = new ListNode(2);
        head.next = node2;
        node2.next = node3;

        printListNode(head);

        head = deleteDuplicates(head);

        printListNode(head);

        head = new ListNode(1);
        node2 = new ListNode(1);
        node3 = new ListNode(2);
        ListNode node4 = new ListNode(3);
        ListNode node5 = new ListNode(3);
        head.next = node2;
        node2.next = node3;
        node3.next = node4;
        node4.next = node5;

        printListNode(head);

        head = deleteDuplicates(head);

        printListNode(head);
    }
}
