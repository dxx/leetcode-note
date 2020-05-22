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
    reverseNode := &ListNode{}
    newLastNode := reverseNode
    firstNode, tempNode := head, head
    i := 1
    for tempNode != nil {
        next := tempNode.Next
        // 遍历指定个数节点开始翻转
        if i % k == 0 {
            // 翻转, 返回新的头结点, 然后拼接在新的节点后
            newLastNode.Next = reverse(firstNode, tempNode)
            // 修改新链表最后一个节点
            newLastNode = firstNode

            // 记录下一次要翻转的起始结点
            firstNode = next
        }

        i++
        tempNode = next
    }
    return reverseNode.Next
}

// 给定起始节点和结束节点翻转链表，返回新的起始节点
func reverse(first *ListNode, last *ListNode) *ListNode {
    reverseNode := &ListNode{}
    temp := first
    for temp != last {
        next := temp.Next
        temp.Next = reverseNode.Next
        reverseNode.Next = temp
        temp = next
    }
    first.Next = last.Next
    temp.Next = reverseNode.Next
    return temp
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
