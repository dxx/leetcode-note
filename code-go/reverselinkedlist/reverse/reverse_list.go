package main

import (
    "fmt"
    "strings"
)

// 反转链表
// 反转一个单链表。

// 示例
// 输入: 1->2->3->4->5
// 输出: 5->4->3->2->1

type ListNode struct {
    Val int
    Next *ListNode
}

func reverseList(head *ListNode) *ListNode {
    reverseNode := &ListNode{}
    current := head
    for current != nil {
        // 记录下一个节点
        next := current.Next
        // 将当前节点指向翻转后的第一个节点
        current.Next = reverseNode.Next
        // 修改新的头节点的下一个节点
        reverseNode.Next = current

        // 指向下一个节点
        current = next
    }
    return reverseNode.Next
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
    head := &ListNode{Val: 1}
    node2 := &ListNode{Val: 2}
    node3 := &ListNode{Val: 3}
    node4 := &ListNode{Val: 4}
    node5 := &ListNode{Val: 5}
    head.Next = node2
    node2.Next = node3
    node3.Next = node4
    node4.Next = node5

    printListNode(head)

    head = reverseList(head)

    printListNode(head)
}
