package validatebinarysearchtree

import (
    "fmt"
    "testing"
)

func TestIsValidBST(t *testing.T) {
    root := &TreeNode{Val: 2}
    node1 := &TreeNode{Val: 1}
    node2 := &TreeNode{Val: 3}
    root.Left = node1
    root.Right = node2

    preOrder(root)

    isValid := isValidBST(root)
    fmt.Println(isValid)

    root = &TreeNode{Val: 5}
    node1 = &TreeNode{Val: 1}
    node2 = &TreeNode{Val: 4}
    node3 := &TreeNode{Val: 3}
    node4 := &TreeNode{Val: 6}
    root.Left = node1
    root.Right = node2
    node2.Left = node3
    node2.Right = node4

    preOrder(root)

    isValid = isValidBST(root)
    fmt.Println(isValid)
}
