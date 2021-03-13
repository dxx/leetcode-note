package com.dxx.linkedlistcycle;

/**
 * 环形链表
 * 给定一个链表，判断链表中是否有环。
 * 为了表示给定链表中的环，我们使用整数 pos 来表示链表尾连接到链表中的位置（索引从 0 开始）。
 * 如果 pos 是 -1，则在该链表中没有环。
 *
 * 进阶: 你能用 O(1)（即常量）内存解决此问题吗？
 *
 * 示例
 * 输入: head = [3,2,0,-4], pos = 1
 * 输出: true
 * 解释: 链表中有一个环，其尾部连接到第二个节点。
 *
 * 输入: head = [1,2], pos = 0
 * 输出: true
 * 解释: 链表中有一个环，其尾部连接到第一个节点。
 *
 * 输入: head = [1], pos = -1
 * 输出: false
 * 解释: 链表中没有环。
 */
public class Main {

    public static class ListNode {
        public int val;
        public ListNode next;
        public ListNode(int val) {
            this.val = val;
        }
    }

    public static boolean hasCycle(ListNode head) {
        ListNode fast = head, slow = head;
        while (fast != null && fast.next != null) {
            // 快指针走两个节点
            fast = fast.next.next;
            // 慢指针走一个节点
            slow = slow.next;
            // 如果有环, 快指针一定会追上慢指针
            if (fast == slow) {
                return true;
            }
        }
        return false;
    }

    public static void main(String[] args) {
        ListNode head = new ListNode(3);
        ListNode node2 = new ListNode(2);
        ListNode node3 = new ListNode(0);
        ListNode node4 = new ListNode(-4);
        head.next = node2;
        node2.next = node3;
        node3.next = node4;
        node4.next = node2;

        boolean result = hasCycle(head);
        System.out.println(result);

        head = new ListNode(1);
        node2 = new ListNode(2);
        head.next = node2;
        node2.next = head;

        result = hasCycle(head);
        System.out.println(result);

        head = new ListNode(1);

        result = hasCycle(head);
        System.out.println(result);
    }
}
