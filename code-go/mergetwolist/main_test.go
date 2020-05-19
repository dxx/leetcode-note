package mergetwolist

import "testing"

func TestMergeTwoLists(t *testing.T) {
    l1 := &ListNode{Val: 1}
    node1 := &ListNode{Val: 2}
    node2 := &ListNode{Val: 4}
    l1.Next = node1
    node1.Next = node2

    l2 := &ListNode{Val: 1}
    node3 := &ListNode{Val: 3}
    node4 := &ListNode{Val: 4}
    l2.Next = node3
    node3.Next = node4

    printListNode(l1)
    printListNode(l2)

    newNode := mergeTwoLists(l1, l2)
    printListNode(newNode)
}
