package com.dxx.twonumaddition;

/**
 * 两数相加
 * 给出两个 **非空** 的链表用来表示两个非负的整数。其中，它们
 * 各自的位数是按照 **逆序** 的方式存储的，并且它们的每个节点
 * 只能存储 **一位** 数字。
 * 如果，我们将这两个数相加起来，则会返回一个新的链表来表示它们的和。
 * 您可以假设除了数字 0 之外，这两个数都不会以 0 开头。
 *
 * 示例
 * 输入: (2 -> 4 -> 3) + (5 -> 6 -> 4)
 * 输出: 7 -> 0 -> 8
 * 原因: 342 + 465 = 807
 *
 * 输入: (2 -> 4 -> 3) + (5 -> 6)
 * 输出: 7 -> 0 -> 4
 * 原因: 342 + 65 = 407
 *
 * 输入: (2 -> 4) + (5 -> 6)
 * 输出: 7 -> 0 -> 1
 * 原因: 42 + 65 = 107
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
        public String toString() {
            StringBuilder sb = new StringBuilder();
            ListNode node = this;
            while (node != null) {
                sb.append(node.val).append("->");
                node = node.next;
            }
            sb.deleteCharAt(sb.length() - 1);
            sb.deleteCharAt(sb.length() - 1);
            return sb.toString();
        }
    }

    public static ListNode addTowNumbers(ListNode l1, ListNode l2) {
        ListNode newNumNode = new ListNode(0);
        ListNode prevNode = newNumNode;
        ListNode node1, node2;
        node1 = l1;
        node2 = l2;
        int val, carry = 0;

        while (node1 != null || node2 != null) {
            int val1 = node1 != null ? node1.val : 0;
            int val2 = node2 != null ? node2.val : 0;
            // 两个节点的值加上上一次相加的进位
            val = val1 + val2 + carry;
            // 获取进位
            carry = val / 10;
            // 创建新的节点
            ListNode newNode = new ListNode(val % 10);
            // 将新的节点添加到上一个节点末尾
            prevNode.next = newNode;
            // 记录上一个节点
            prevNode = newNode;

            if (node1 != null) {
                node1 = node1.next;
            }
            if (node2 != null) {
                node2 = node2.next;
            }
        }

        // 如果有进位增加一个新的节点
        if (carry > 0) {
            prevNode.next = new ListNode(carry);
        }

        return newNumNode.next;
    }

    public static void main(String[] args) {
        ListNode l1 = new ListNode(2);
        ListNode node1 = new ListNode(4);
        ListNode node2 = new ListNode(3);
        l1.next = node1;
        node1.next = node2;

        ListNode l2 = new ListNode(5);
        ListNode node3 = new ListNode(6);
        ListNode node4 = new ListNode(4);
        l2.next = node3;
        node3.next = node4;

        System.out.println(l1);
        System.out.println(l2);

        ListNode newNode = addTowNumbers(l1, l2);
        System.out.println(newNode);
    }
}
