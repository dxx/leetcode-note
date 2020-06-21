package sortlist

import (
    "fmt"
    "strings"
)

// 排序链表
// 在 O(n log n) 时间复杂度和常数级空间复杂度下，对链表进行排序。

// 示例
// 输入: 4->2->1->3
// 输出: 1->2->3->4

// 输入: -1->5->3->4->0
// 输出: -1->0->3->4->5

type ListNode struct {
    Val int
    Next *ListNode
}

func sortList(head *ListNode) *ListNode {
    return sort(head, nil)
}

// 分解排序
func sort(head, tail *ListNode) *ListNode {
    if head == tail {
        return head
    }
    middleNode := getMiddleNode(head, tail)
    // 保存下一个节点
    next := middleNode.Next
    // 断开链表
    middleNode.Next = nil
    // 分解左边
    head1 := sort(head, middleNode)
    // 分解右边
    head2 := sort(next, tail)
    // 将两个有序的链表合并成一个链表
    return merge(head1, head2)
}

// 合并有序链表
func merge(head1, head2 *ListNode) *ListNode {
    current1, current2 := head1, head2

    var appendNode *ListNode
    newHead := &ListNode{Val: -1}
    last := newHead
    for current1 != nil && current2 != nil {
        if current1.Val <= current2.Val {
            appendNode = current1
            current1 = current1.Next
        } else {
            appendNode = current2
            current2 = current2.Next
        }
        last.Next = appendNode
        last = appendNode
    }

    // 左边还有剩余节点
    if current1 != nil {
        last.Next = current1
    }
    // 右边还有剩余节点
    if current2 != nil {
        last.Next = current2
    }

    return newHead.Next
}

// 获取链表的中间节点
func getMiddleNode(head, tail *ListNode) *ListNode {
    fast, slow := head, head
    for fast != tail && fast.Next != tail {
        fast = fast.Next.Next
        slow = slow.Next
    }
    return slow
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
