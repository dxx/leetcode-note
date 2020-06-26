package reverse

import "testing"

func TestReverseList(t *testing.T) {
	head := &ListNode{Val: 1}
	node2 := &ListNode{Val: 2}
	node3 := &ListNode{Val: 3}
	node4 := &ListNode{Val: 4}
	node5 := &ListNode{Val: 5}
	head.Next = node2
	node2.Next = node3
	node3.Next = node4
	node4.Next = node5

	printListNode(head)

	head = reverseList(head)

	printListNode(head)
}
