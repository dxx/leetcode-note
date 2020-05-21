package swapnodesinpairs

import (
    "fmt"
    "strings"
)

// 两两交换链表中的节点
// 给定一个链表，两两交换其中相邻的节点，并返回交换后的链表。**你不能只是单纯的改变节点内部的值**，而是需要实际的进行节点交换。

// 示例
// 输入: 1->2->3->4
// 输出: 2->1->4->3

type ListNode struct {
    Val int
    Next *ListNode
}

func printListNode(node *ListNode) {
    var str = ""
    for node != nil {
        str += fmt.Sprintf("%d->", node.Val)
        node = node.Next
    }
    str = strings.TrimSuffix(str, "->")
    fmt.Println(str)
}

func swapPairs(head *ListNode) *ListNode {
    swapHeadNode := &ListNode{} // 新的头结点，用来交换节点
    lastNode := swapHeadNode // 保存新节点中之后一个节点
    lastNode.Next = head // 当 head 只有一个节点时，默认就是 head
    current := head
    var prevNode, nextNode *ListNode
    // 每次遍历两个节点
    for current != nil && current.Next != nil {
        prevNode = current // 前一个节点
        nextNode = current.Next // 后一个节点

        // 保存下一次遍历时的节点
        next := nextNode.Next

        // 新的链表中最后一个节点指向后一个节点
        lastNode.Next = nextNode

        // 先将上一个节点指向原链表中的下一次遍历时的节点
        prevNode.Next = next

        // 新的链表中最后一个节点指向原链表中的上一个节点
        nextNode.Next = prevNode
        // 记录新的链表中最后一个节点
        lastNode = prevNode

        current = next
    }
    return swapHeadNode.Next
}
