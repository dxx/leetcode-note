package removenthfromend

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
