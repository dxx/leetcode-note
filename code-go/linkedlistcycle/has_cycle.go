package main

import "fmt"

// 环形链表
// 给定一个链表，判断链表中是否有环。
// 为了表示给定链表中的环，我们使用整数 pos 来表示链表尾连接到链表中的位置（索引从 0 开始）。
// 如果 pos 是 -1，则在该链表中没有环。

// 进阶: 你能用 O(1)（即常量）内存解决此问题吗？

// 示例
// 输入: head = [3,2,0,-4], pos = 1
// 输出: true
// 解释: 链表中有一个环，其尾部连接到第二个节点。

// 输入: head = [1,2], pos = 0
// 输出: true
// 解释: 链表中有一个环，其尾部连接到第一个节点。

// 输入: head = [1], pos = -1
// 输出: false
// 解释: 链表中没有环。

type ListNode struct {
    Val int
    Next *ListNode
}

func hasCycle(head *ListNode) bool {
    fast, slow := head, head
    for fast != nil && fast.Next != nil {
        // 快指针走两个节点
        fast = fast.Next.Next
        // 慢指针走一个节点
        slow = slow.Next
        // 如果有环, 快指针一定会追上慢指针
        if fast == slow {
            return true
        }
    }
    return false
}

func main() {
    head := &ListNode{Val: 3}
    node2 := &ListNode{Val: 2}
    node3 := &ListNode{Val: 0}
    node4 := &ListNode{Val: -4}
    head.Next = node2
    node2.Next = node3
    node3.Next = node4
    node4.Next = node2

    result := hasCycle(head)
    fmt.Println(result)

    head = &ListNode{Val: 1}
    node2 = &ListNode{Val: 2}
    head.Next = node2
    node2.Next = head

    result = hasCycle(head)
    fmt.Println(result)

    head = &ListNode{Val: 1}

    result = hasCycle(head)
    fmt.Println(result)
}
