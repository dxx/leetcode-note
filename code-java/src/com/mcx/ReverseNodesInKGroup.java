package com.mcx;

/**
 * Created by mcx on 2020-08-04.
 *
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
public class ReverseNodesInKGroup {

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
        ListNode reverseNode = new ListNode(0);
        ListNode newLastNode = reverseNode;
        ListNode firstNode = head, tempNode = head;
        int i = 1;
        while (tempNode != null) {
            ListNode next = tempNode.next;
            // 遍历指定个数节点开始翻转
            if (i % k == 0) {
                // 翻转, 返回新的头结点, 然后拼接在新的节点后
                newLastNode.next = reverse(firstNode, tempNode);
                // 修改新链表最后一个节点
                newLastNode = firstNode;

                // 记录下一次要翻转的起始结点
                firstNode = next;
            }

            i++;
            tempNode = next;
        }
        return reverseNode.next;
    }

    /**
     * 给定起始节点和结束节点翻转链表，返回新的头节点
     */
    private static ListNode reverse(ListNode first, ListNode last) {
        ListNode reverseNode = new ListNode(0);
        ListNode temp = first;
        while (temp != last) {
            ListNode next = temp.next;
            temp.next = reverseNode.next;
            reverseNode.next = temp;
            temp = next;
        }
        first.next = last.next;
        temp.next = reverseNode.next;
        return temp;
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
