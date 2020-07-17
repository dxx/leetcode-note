package main

import (
    "fmt"
    "strings"
)

// 删除链表的倒数第N个节点
// 给定一个链表，删除链表的倒数第 n 个节点，并且返回链表的头结点。

// 说明:
// 给定的 n 保证是有效的。

// 示例
// 输入: 1->2->3->4->5, n = 2
// 输出: 1->2->3->5

type ListNode struct {
    Val int
    Next *ListNode
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

func removeNthFromEnd(head *ListNode, n int) *ListNode {
    if head == nil || n <= 0 {
        return head
    }

    var prevNode *ListNode
    // 快指针，表示需要循环多少次
    fast := head
    // 慢指针，表示倒数第 n 个节点
    slow := head
    for fast != nil {
        if n > 0 {
            // 先将快指针走到第 n 个节点
            fast = fast.Next
            n--
            continue
        }
        // 记录上一个节点
        prevNode = slow

        // 快慢指针同时走
        fast = fast.Next
        slow = slow.Next
    }

    // 当第一次 fast 指针全部走完时 prevNode = slow 未执行
    if prevNode != nil {
        prevNode.Next = prevNode.Next.Next
    } else {
        // 删除的是第一个节点
        head = head.Next
    }

    return head
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

    n := 2

    printListNode(head)
    fmt.Printf("n = %d\n", n)

    head = removeNthFromEnd(head, n)

    printListNode(head)
}
