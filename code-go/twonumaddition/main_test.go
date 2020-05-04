package twonumaddition

import "testing"

func TestAddTwoNumbers(t *testing.T) {
    l1 := &NumNode{Val: 2}
    node1 := &NumNode{Val: 4}
    node2 := &NumNode{Val: 3}
    l1.Next = node1
    node1.Next = node2

    l2 := &NumNode{Val: 5}
    node3 := &NumNode{Val: 6}
    node4 := &NumNode{Val: 4}
    l2.Next = node3
    node3.Next = node4

    t.Log(l1)
    t.Log(l2)

    newNode := addTwoNumbers(l1, l2)
    t.Log(newNode)
}
