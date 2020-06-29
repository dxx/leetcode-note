package com.mcx;

import java.util.Arrays;

/**
 * Created by mcx on 2020-06-29.
 *
 * 合并 K 个排序链表
 * 合并 k 个排序链表，返回合并后的排序链表。
 *
 * 示例
 * 输入: [1->4->5, 1->3->4, 2->6]
 * 输出: 1->1->2->3->4->4->5->6
 */
public class MergeKList {
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

    public static ListNode mergeKLists(ListNode[] listNodes) {
        if (listNodes == null || listNodes.length == 0) {
            return null;
        }

        // 顺序合并
//        ListNode listNode = null;
//        for (ListNode node : listNodes) {
//            listNode = merge(listNode, node);
//        }
//        return listNode;

        // 分治合并
        return divide(listNodes, 0, listNodes.length - 1);
    }

    // 分解
    private static ListNode divide(ListNode[] listNodes, int start, int end) {
        if (start < end) {
            // 计算中间位置
            int mid = (start + end) >> 1;
            // 分解左边
            ListNode listNode1 = divide(listNodes, start, mid);
            // 分解右边
            ListNode listNode2 = divide(listNodes, mid + 1, end);
            // 合并左边和右边得到新的链表
            return merge(listNode1, listNode2);
        }
        // 当分解到最小时，有 start == end
        return listNodes[end];
    }

    // 合并
    private static ListNode merge(ListNode l1, ListNode l2) {
        ListNode newListNode = null;
        ListNode prevNewNode = null;
        ListNode appendNode = null;
        ListNode node1 = l1;
        ListNode node2 = l2;
        while (node1 != null && node2 != null) {
            if (node1.val <= node2.val) { // 判断左边链表节点是否小于等于右边链表节点
                appendNode = node1;
                // 左边当前链表节点后移
                node1 = node1.next;
            } else { // 判断左边链表节点是大于右边链表节点
                appendNode = node2;
                // 右边当前链表节点后移
                node2 = node2.next;
            }
            // 判断是否添加第一个节点
            if (prevNewNode == null) {
                newListNode = appendNode;
            } else {
                // 将下一个节点添加到上一个节点的后面
                prevNewNode.next = appendNode;
            }
            // 记录上一个节点
            prevNewNode = appendNode;
        }

        // 左边链表还有剩余节点，直接添加到末尾
        if (node1 != null) {
            if (prevNewNode == null) {
                newListNode = node1;
            } else {
                prevNewNode.next = node1;
            }
        }

        // 右边链表还有剩余节点，直接添加到末尾
        if (node2 != null) {
            if (prevNewNode == null) {
                newListNode = node2;
            } else {
                prevNewNode.next = node2;
            }
        }

        return newListNode;
    }

    public static void main(String[] args) {
        ListNode l1 = new ListNode(1);
        ListNode node1 = new ListNode(4);
        ListNode node2 = new ListNode(5);
        l1.next = node1;
        node1.next = node2;

        ListNode l2 = new ListNode(1);
        ListNode node3 = new ListNode(3);
        ListNode node4 = new ListNode(4);
        l2.next = node3;
        node3.next = node4;

        ListNode l3 = new ListNode(2);
        ListNode node5 = new ListNode(6);
        l3.next = node5;

        ListNode[] listNodes = {l1, l2, l3};
        System.out.println(Arrays.toString(listNodes));

        ListNode newNode = mergeKLists(listNodes);
        System.out.println(newNode);
    }
}
