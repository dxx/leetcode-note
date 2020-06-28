package main

import (
    "fmt"
    "strings"
)

// 合并两个有序链表
// 将两个升序链表合并为一个新的升序链表并返回。新链表是通过拼接给定的两个链表的所有节点组成的。

// 示例
// 输入：1->2->4, 1->3->4
// 输出：1->1->2->3->4->4

// 单链表结构体
type ListNode struct {
    Val int
    Next *ListNode
}

func mergeTwoLists(l1 *ListNode, l2 *ListNode) *ListNode {
    var prevNewNode *ListNode
    var newListNode *ListNode
    var appendNode *ListNode
    node1 := l1
    node2 := l2
    for node1 != nil && node2 != nil {
        if node1.Val <= node2.Val { // 判断左边链表节点是否小于等于右边链表节点
            appendNode = node1
            // 左边当前链表节点后移
            node1 = node1.Next
        } else if node1.Val > node2.Val { // 判断左边链表节点是大于右边链表节点
            appendNode = node2
            // 右边当前链表节点后移
            node2 = node2.Next
        }
        // 判断是否添加第一个节点
        if prevNewNode == nil {
            newListNode = appendNode
        } else {
            // 将下一个节点添加到上一个节点的后面
            prevNewNode.Next = appendNode
        }
        // 记录上一个节点
        prevNewNode = appendNode
    }

    // 左边链表还有剩余节点
    if node1 != nil {
        if prevNewNode == nil {
            newListNode = node1
        } else {
            prevNewNode.Next = node1
        }
    }

    // 右边链表还有剩余节点
    if node2 != nil {
        if prevNewNode == nil {
            newListNode = node2
        } else {
            prevNewNode.Next = node2
        }
    }
    return newListNode
}

// 打印链表
func printListNode(node *ListNode) {
    var str = ""
    for node != nil {
        str += fmt.Sprintf("%d->", node.Val)
        node = node.Next
    }
    str = strings.TrimSuffix(str, "->")
    fmt.Println(str)
}

func main() {
    l1 := &ListNode{Val: 1}
    node1 := &ListNode{Val: 2}
    node2 := &ListNode{Val: 4}
    l1.Next = node1
    node1.Next = node2

    l2 := &ListNode{Val: 1}
    node3 := &ListNode{Val: 3}
    node4 := &ListNode{Val: 4}
    l2.Next = node3
    node3.Next = node4

    printListNode(l1)
    printListNode(l2)

    newNode := mergeTwoLists(l1, l2)
    printListNode(newNode)
}
