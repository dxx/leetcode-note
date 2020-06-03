package reverse2

import (
    "fmt"
    "strings"
)

// 反转链表 II
// 反转从位置 m 到 n 的链表。请使用一趟扫描完成反转。
// 说明:
// 1 ≤ m ≤ n ≤ 链表长度。

// 示例
// 输入: 1->2->3->4->5, m = 2, n = 4
// 输出: 1->4->3->2->5

type ListNode struct {
    Val int
    Next *ListNode
}

func reverseBetween(head *ListNode, m int, n int) *ListNode {
    var beforeRevNode *ListNode
    current := head

    i := 1
    for current != nil {
        if i == m - 1 {
            // 记录开始翻转之前的节点
            beforeRevNode = current
        }
        // 找到开始翻转的节点
        if i == m {
            break
        }
        current = current.Next
        i++
    }

    if current == nil {
        return head
    }

    reverseNode := &ListNode{}
    temp := current
    for current != nil && i <= n {
        next := current.Next
        current.Next = reverseNode.Next
        reverseNode.Next = current

        current = next
        i++
    }
    // 将翻转后的开始节点的下一个节点指向翻转尾节点的下一个节点
    temp.Next = current

    // 表示头节点没有翻转
    if beforeRevNode != nil {
        // 将翻转后的链表拼接在翻转之前的节点后面
        beforeRevNode.Next = reverseNode.Next
        return head
    } else { // 头节点翻转
        return reverseNode.Next
    }
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
