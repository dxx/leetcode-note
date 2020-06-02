package remove

import "testing"

func TestDeleteDuplicates(t *testing.T) {
    head := &ListNode{Val: 1}
    node2 := &ListNode{Val: 1}
    node3 := &ListNode{Val: 2}
    head.Next = node2
    node2.Next = node3

    printListNode(head)

    head = deleteDuplicates(head)

    printListNode(head)

    head = &ListNode{Val: 1}
    node2 = &ListNode{Val: 1}
    node3 = &ListNode{Val: 2}
    node4 := &ListNode{Val: 3}
    node5 := &ListNode{Val: 3}
    head.Next = node2
    node2.Next = node3
    node3.Next = node4
    node4.Next = node5

    printListNode(head)

    head = deleteDuplicates(head)

    printListNode(head)
}
