package balancedbinarytree

import (
	"fmt"
	"testing"
)

func TestIsBalanced(t *testing.T) {
	node1 := &TreeNode{Val: 3}
	node2 := &TreeNode{Val: 9}
	node3 := &TreeNode{Val: 20}
	node4 := &TreeNode{Val: 15}
	node5 := &TreeNode{Val: 7}
	node1.Left = node2
	node1.Right = node3
	node3.Left = node4
	node3.Right = node5

	infixOrder(node1)

	result := isBalanced(node1)

	fmt.Println(result)

	node1 = &TreeNode{Val: 1}
	node2 = &TreeNode{Val: 2}
	node3 = &TreeNode{Val: 2}
	node4 = &TreeNode{Val: 3}
	node5 = &TreeNode{Val: 3}
	node6 := &TreeNode{Val: 4}
	node7 := &TreeNode{Val: 4}
	node1.Left = node2
	node1.Right = node3
	node2.Left = node4
	node2.Right = node5
	node4.Left = node6
	node4.Right = node7

	infixOrder(node1)

	result = isBalanced(node1)

	fmt.Println(result)
}
