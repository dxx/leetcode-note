package mergeklist

import (
    "fmt"
    "testing"
)

func TestMergeKLists(t *testing.T) {
    l1 := &ListNode{Val: 1}
    node1 := &ListNode{Val: 4}
    node2 := &ListNode{Val: 5}
    l1.Next = node1
    node1.Next = node2

    l2 := &ListNode{Val: 1}
    node3 := &ListNode{Val: 3}
    node4 := &ListNode{Val: 4}
    l2.Next = node3
    node3.Next = node4

    l3 := &ListNode{Val: 2}
    node5 := &ListNode{Val: 6}
    l3.Next = node5


    lists := []*ListNode{
        l1, l2, l3,
    }

    fmt.Println(lists)

    newNode := mergeKLists(lists)
    fmt.Println(newNode)
}
