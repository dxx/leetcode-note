package com.mcx.sortlist;

/**
 * 排序链表
 * 在 O(n log n) 时间复杂度和常数级空间复杂度下，对链表进行排序。
 *
 * 示例
 * 输入: 4->2->1->3
 * 输出: 1->2->3->4
 *
 * 输入: -1->5->3->4->0
 * 输出: -1->0->3->4->5
 */
public class Main {

    public static class ListNode {
        public int val;
        public ListNode next;
        public ListNode(int val) {
            this.val = val;
        }
    }

    public static ListNode sortList(ListNode head) {
        return sort(head, null);
    }

    public static ListNode sort(ListNode head, ListNode tail) {
        if (head == tail) {
            return head;
        }
        ListNode middleNode = getMiddleNode(head, tail);
        // 保存下一个节点
        ListNode next = middleNode.next;
        // 断开链表
        middleNode.next = null;
        // 分解左边
        ListNode head1 = sort(head, middleNode);
        // 分解右边
        ListNode head2 = sort(next, tail);
        // 将两个有序的链表合并成一个链表
        return merge(head1, head2);
    }

    /**
     * 合并有序链表
     */
    public static ListNode merge(ListNode head1, ListNode head2) {
        ListNode current1 = head1, current2 = head2;
        ListNode appendNode = null;
        ListNode newHead = new ListNode(-1);
        ListNode last = newHead;
        while (current1 != null && current2 != null) {
            if (current1.val <= current2.val) {
                appendNode = current1;
                current1 = current1.next;
            } else {
                appendNode = current2;
                current2 = current2.next;
            }
            last.next = appendNode;
            last = appendNode;
        }

        // 左边还有剩余节点
        if (current1 != null) {
            last.next = current1;
        }
        // 右边还有剩余节点
        if (current2 != null) {
            last.next = current2;
        }

        return newHead.next;
    }

    /**
     * 获取链表的中间节点
     */
    public static ListNode getMiddleNode(ListNode head, ListNode tail) {
        ListNode fast = head, slow = head;
        while (fast != tail && fast.next != tail) {
            fast = fast.next.next;
            slow = slow.next;
        }
        return slow;
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
        ListNode node1 = new ListNode(4);
        ListNode node2 = new ListNode(2);
        ListNode node3 = new ListNode(1);
        ListNode node4 = new ListNode(3);
        node1.next = node2;
        node2.next = node3;
        node3.next = node4;

        printListNode(node1);

        ListNode node = sortList(node1);

        printListNode(node);

        node1 = new ListNode(-1);
        node2 = new ListNode(5);
        node3 = new ListNode(3);
        node4 = new ListNode(4);
        ListNode node5 = new ListNode(0);
        node1.next = node2;
        node2.next = node3;
        node3.next = node4;
        node4.next = node5;

        printListNode(node1);

        node = sortList(node1);

        printListNode(node);
    }
}
