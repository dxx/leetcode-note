package reversenodesinkgroup

import (
    "fmt"
    "strings"
)

// K 个一组翻转链表
// 给你一个链表，每 k 个节点一组进行翻转，请你返回翻转后的链表。
// k 是一个正整数，它的值小于或等于链表的长度。
// 如果节点总数不是 k 的整数倍，那么请将最后剩余的节点保持原有顺序。

// 示例
// 输入: 1->2->3->4->5
// 当 k = 2 时, 返回: 2->1->4->3->5
// 当 k = 3 时, 返回: 3->2->1->4->5

type ListNode struct {
    Val int
    Next *ListNode
}

func reverseKGroup(head *ListNode, k int) *ListNode {
    tail := head
    // 寻找从 head 节点开始的第 k 个节点的下一个节点
    for i := 0; i < k; i++ {
        // 不足 k 个节点不翻转
        if tail == nil {
            return head
        }
        tail = tail.Next
    }
    // 翻转 head 节点，不翻转 tail
    nextHead := reverse(head, tail)
    // 递归翻转，连接新的头结点
    head.Next = reverseKGroup(tail, k)
    return nextHead
}

// 给定起始节点和尾节点翻转链表，返回新的起始节点（不翻转尾节点）
func reverse(head *ListNode, tail *ListNode) *ListNode {
    var last *ListNode
    temp := head
    for temp != tail {
        // 下一个节点
        next := temp.Next
        // 当前节点的下一个节点指向尾节点
        temp.Next = last
        // 记录下一个节点
        last = temp
        temp = next
    }
    return last
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
