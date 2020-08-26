package main

import (
    "fmt"
    "math"
    "strings"
)

// 对链表进行插入排序
// 从第一个元素开始，该链表可以被认为已经部分排序。每次迭代时，从输入数据中移除一个元素，并原地将其插入到已排好序的链表中。
// 插入排序算法:
//   1.插入排序是迭代的，每次只移动一个元素，直到所有元素可以形成一个有序的输出列表。
//   2.每次迭代中，插入排序只从输入数据中移除一个待排序的元素，找到它在序列中适当的位置，并将其插入。
//   3.重复直到所有输入数据插入完为止。

// 示例
// 输入: 4->2->1->3
// 输出: 1->2->3->4

// 输入: -1->5->3->4->0
// 输出: -1->0->3->4->5

type ListNode struct {
    Val int
    Next *ListNode
}

func insertionSortList(head *ListNode) *ListNode {
    newHead := &ListNode{Val: math.MinInt32}
    newHead.Next = head
    current := newHead
    for current != nil && current.Next != nil {
        // 用来插入的节点
        next := current.Next
        temp := newHead
        // 从头节点开始循环到 next 的前一个节点
        for temp != next {
            // 将用来插入节点一次和每个节点比较
            if next.Val < temp.Next.Val {
                // 在节点插入到有序的子链表前, 先将 current 节点指向用来插入的节点的下一个节点
                current.Next = next.Next

                // 将节点插入到合适位置
                next.Next = temp.Next
                temp.Next = next
                break
            }
            temp = temp.Next
        }
        // 表示没有节点插入, 继续从下一个节点开始走
        // 如果有节点插入, current 节点已经正确指向了下一个节点
        if temp == next {
            current = current.Next
        }
    }
    return newHead.Next
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

func main() {
    node1 := &ListNode{Val: 4}
    node2 := &ListNode{Val: 2}
    node3 := &ListNode{Val: 1}
    node4 := &ListNode{Val: 3}
    node1.Next = node2
    node2.Next = node3
    node3.Next = node4

    printListNode(node1)

    node := insertionSortList(node1)

    printListNode(node)

    node1 = &ListNode{Val: -1}
    node2 = &ListNode{Val: 5}
    node3 = &ListNode{Val: 3}
    node4 = &ListNode{Val: 4}
    node5 := &ListNode{Val: 0}
    node1.Next = node2
    node2.Next = node3
    node3.Next = node4
    node4.Next = node5

    printListNode(node1)

    node = insertionSortList(node1)

    printListNode(node)
}
