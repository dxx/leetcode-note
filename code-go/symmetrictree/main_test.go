package symmetrictree

import "testing"

func TestIsSymmetric(t *testing.T) {
    root := &TreeNode{Val: 1}
    node2 := &TreeNode{Val: 2}
    node3 := &TreeNode{Val: 2}
    node4 := &TreeNode{Val: 3}
    node5 := &TreeNode{Val: 3}
    node6 := &TreeNode{Val: 4}
    node7 := &TreeNode{Val: 4}
    root.Left = node2
    root.Right = node3
    node2.Left = node4
    node2.Right = node6
    node3.Left = node7
    node3.Right = node5

    result := isSymmetric(root)

    t.Log(result)

    root = &TreeNode{Val: 1}
    node2 = &TreeNode{Val: 2}
    node3 = &TreeNode{Val: 2}
    node4 = &TreeNode{Val: 3}
    node5 = &TreeNode{Val: 3}
    root.Left = node2
    root.Right = node3
    node2.Right = node4
    node3.Right = node5

    result = isSymmetric(root)

    t.Log(result)
}
