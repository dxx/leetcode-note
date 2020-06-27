package convertsortedlisttobinarysearchtree

import "testing"

func TestSortedListToBST(t *testing.T) {
    node1 := &ListNode{Val: -10}
    node2 := &ListNode{Val: -3}
    node3 := &ListNode{Val: 0}
    node4 := &ListNode{Val: 5}
    node5 := &ListNode{Val: 9}
    node1.Next = node2
    node2.Next = node3
    node3.Next = node4
    node4.Next = node5

    tree := sortedListToBST(node1)

    infixOrder(tree)
}
