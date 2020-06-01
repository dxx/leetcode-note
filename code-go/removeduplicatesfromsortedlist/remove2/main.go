package remove2

import (
    "fmt"
    "strings"
)

// 删除排序链表中的重复元素 II
// 给定一个排序链表，删除所有含有重复数字的节点，只保留原始链表中 没有重复出现 的数字。

// 示例
// 输入: 1->2->3->3->4->4->5
// 输出: 1->2->5

// 输入: 1->1->1->2->3
// 输出: 2->3

type ListNode struct {
    Val int
    Next *ListNode
}

func deleteDuplicates(head *ListNode) *ListNode {
    newHead := &ListNode{Val: 0}
    newHead.Next = head
    current := newHead
    for current != nil && current.Next != nil {
        next := current.Next
        // 如果到最后一个节点或者下一个节点和下下一个节点不同
        if next.Next == nil || next.Next.Val != next.Val {
            current = next
            continue
        }
        // 下一个节点相同, 一直遍历直到不相同的节点
        for next.Next != nil && next.Next.Val == next.Val {
            next = next.Next
        }
        // 跳过相同的节点, 指向下一个不同的节点
        current.Next = next.Next
    }
    return newHead.Next
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
