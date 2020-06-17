package linkedlistcycle

import "testing"

func TestHasCycle(t *testing.T) {
	head := &ListNode{Val: 3}
	node2 := &ListNode{Val: 2}
	node3 := &ListNode{Val: 0}
	node4 := &ListNode{Val: -4}
	head.Next = node2
	node2.Next = node3
	node3.Next = node4
	node4.Next = node2

	result := hasCycle(head)
	t.Log(result)

	head = &ListNode{Val: 1}
	node2 = &ListNode{Val: 2}
	head.Next = node2
	node2.Next = head

	result = hasCycle(head)
	t.Log(result)

	head = &ListNode{Val: 1}

	result = hasCycle(head)
	t.Log(result)
}
