package mergeklist

import (
    "fmt"
    "strings"
)

// 合并 K 个排序链表
// 合并 k 个排序链表，返回合并后的排序链表。

// 示例
// 输入: [1->4->5, 1->3->4, 2->6]
// 输出: 1->1->2->3->4->4->5->6

// 单链表结构体
type ListNode struct {
    Val int
    Next *ListNode
}

func (listNode *ListNode) String() string {
    node := listNode
    var str = ""
    for node != nil {
        str += fmt.Sprintf("%d->", node.Val)
        node = node.Next
    }
    str = strings.TrimSuffix(str, "->")
    return str
}

func mergeKLists(lists []*ListNode) *ListNode {
    if len(lists) == 0 {
        return nil
    }

    // 顺序合并
    //var listNode *ListNode
    //for i := 0; i < len(lists); i++ {
    //   listNode = merge(listNode, lists[i])
    //}
    //return listNode

    // 分治合并
    return divide(lists, 0, len(lists) - 1)
}

// 分解
func divide(lists []*ListNode, start, end int) *ListNode {
    if start < end {
        // 计算中间位置
        mid := (start + end) >> 1
        // 分解左边
        listNode1 := divide(lists, start, mid)
        // 分解右边
        listNode2 := divide(lists, mid + 1, end)
        // 合并左边和右边得到新的链表
        return merge(listNode1, listNode2)
    }
    // 当分解到最小时，有 start == end
    // fmt.Println(start == end) // true
    return lists[end]
}

// 合并
func merge(l1 *ListNode, l2 *ListNode) *ListNode {
    var newListNode = &ListNode{Val: 0}
    var prevNewNode = newListNode
    var appendNode *ListNode
    node1 := l1
    node2 := l2
    for node1 != nil && node2 != nil {
        if node1.Val <= node2.Val { // 判断左边链表节点是否小于等于右边链表节点
            appendNode = node1
            // 左边当前链表节点后移
            node1 = node1.Next
        } else { // 判断左边链表节点是大于右边链表节点
            appendNode = node2
            // 右边当前链表节点后移
            node2 = node2.Next
        }

        // 将下一个节点添加到上一个节点的后面
        prevNewNode.Next = appendNode
        // 记录上一个节点
        prevNewNode = appendNode
    }

    // 左边链表还有剩余节点
    if node1 != nil {
        prevNewNode.Next = node1
    }

    // 右边链表还有剩余节点
    if node2 != nil {
        prevNewNode.Next = node2
    }
    return newListNode.Next
}
