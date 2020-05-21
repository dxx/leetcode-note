package swapnodesinpairs

import (
    "fmt"
    "testing"
)

func TestSwapPairs(t *testing.T) {
    head := &ListNode{Val: 1}
    node1 := &ListNode{Val: 2}
    node2 := &ListNode{Val: 3}
    node3 := &ListNode{Val: 4}
    head.Next = node1
    node1.Next = node2
    node2.Next = node3

    fmt.Print("交换前:")
    printListNode(head)

    newNode := swapPairs(head)

    fmt.Print("交换后:")
    printListNode(newNode)
}
