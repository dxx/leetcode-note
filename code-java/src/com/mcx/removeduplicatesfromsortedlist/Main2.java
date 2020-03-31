package com.mcx.removeduplicatesfromsortedlist;

/**
 * 删除排序链表中的重复元素 II
 * 给定一个排序链表，删除所有含有重复数字的节点，只保留原始链表中 没有重复出现 的数字。
 *
 * 示例
 * 输入: 1->2->3->3->4->4->5
 * 输出: 1->2->5
 *
 * 输入: 1->1->1->2->3
 * 输出: 2->3
 */
public class Main2 {

    public static class ListNode {
        public int val;
        public ListNode next;
        public ListNode(int val) {
            this.val = val;
        }
    }

    public static ListNode deleteDuplicates(ListNode head) {
        ListNode newHead = new ListNode(0);
        newHead.next = head;
        ListNode current = newHead;
        while (current != null && current.next != null) {
            ListNode next = current.next;
            // 如果到最后一个节点或者下一个节点和下下一个节点不同
            if (next.next == null || next.next.val != next.val) {
                current = next;
                continue;
            }
            // 下一个节点相同, 一直遍历直到不相同的节点
            while (next.next != null && next.next.val == next.val) {
                next = next.next;
            }
            // 跳过相同的节点, 指向下一个不同的节点
            current.next = next.next;
        }
        return newHead.next;
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
        ListNode node4 = new ListNode(3);
        ListNode node5 = new ListNode(4);
        ListNode node6 = new ListNode(4);
        ListNode node7= new ListNode(5);
        head.next = node2;
        node2.next = node3;
        node3.next = node4;
        node4.next = node5;
        node5.next = node6;
        node6.next = node7;

        printListNode(head);

        head = deleteDuplicates(head);

        printListNode(head);

        head = new ListNode(1);
        node2 = new ListNode(1);
        node3 = new ListNode(1);
        node4 = new ListNode(2);
        node5 = new ListNode(3);
        head.next = node2;
        node2.next = node3;
        node3.next = node4;
        node4.next = node5;

        printListNode(head);

        head = deleteDuplicates(head);

        printListNode(head);
    }
}
