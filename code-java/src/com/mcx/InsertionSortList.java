package com.mcx;

/**
 * Created by mcx on 2020-08-26.
 *
 * 对链表进行插入排序
 * 从第一个元素开始，该链表可以被认为已经部分排序。每次迭代时，从输入数据中移除一个元素，并原地将其插入到已排好序的链表中。
 * 插入排序算法:
 *   1.插入排序是迭代的，每次只移动一个元素，直到所有元素可以形成一个有序的输出列表。
 *   2.每次迭代中，插入排序只从输入数据中移除一个待排序的元素，找到它在序列中适当的位置，并将其插入。
 *   3.重复直到所有输入数据插入完为止。
 *
 * 示例
 * 输入: 4->2->1->3
 * 输出: 1->2->3->4
 *
 * 输入: -1->5->3->4->0
 * 输出: -1->0->3->4->5
 */
public class InsertionSortList {

    public static class ListNode {
        public int val;
        public ListNode next;
        public ListNode(int val) {
            this.val = val;
        }
    }

    public static ListNode insertionSortList(ListNode head) {
        ListNode newHead = new ListNode(Integer.MIN_VALUE);
        newHead.next = head;
        ListNode current = newHead;
        while (current != null && current.next != null) {
            // 用来插入的节点
            ListNode next = current.next;
            ListNode temp = newHead;
            // 从头节点开始循环到 next 的前一个节点
            while (temp != next) {
                // 将用来插入节点一次和每个节点比较
                if (next.val < temp.next.val) {
                    // 在节点插入到有序的子链表前, 先将 current 节点指向用来插入的节点的下一个节点
                    current.next = next.next;

                    // 将节点插入到合适位置
                    next.next = temp.next;
                    temp.next = next;
                    break;
                }
                temp = temp.next;
            }
            // 表示没有节点插入, 继续从下一个节点开始走
            // 如果有节点插入, current 节点已经正确指向了下一个节点
            if (temp == next) {
                current = current.next;
            }
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
        ListNode node1 = new ListNode(4);
        ListNode node2 = new ListNode(2);
        ListNode node3 = new ListNode(1);
        ListNode node4 = new ListNode(3);
        node1.next = node2;
        node2.next = node3;
        node3.next = node4;

        printListNode(node1);

        ListNode node = insertionSortList(node1);

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

        node = insertionSortList(node1);

        printListNode(node);
    }
}
