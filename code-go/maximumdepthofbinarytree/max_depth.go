package main

import "fmt"

// 二叉树的最大深度
// 给定一个二叉树，找出其最大深度。二叉树的深度为根节点到最远叶子节点的最长路径上的节点数。
// 说明: 叶子节点是指没有子节点的节点。

// 示例
// 给定二叉树 [3,9,20,null,null,15,7]
//   3
//  / \
// 9  20
//   /  \
//  15   7
// 它的最大深度是 3 。

type TreeNode struct {
    Val int
    Left *TreeNode
    Right *TreeNode
}

func maxDepth(root *TreeNode) int {
    if root == nil {
        return 0
    }
    // 左子树高度
    lHeight := maxDepth(root.Left)
    // 右子树高度
    rHeight := maxDepth(root.Right)
    // 返回最大高度, 当前节点的高度为 1
    return max(lHeight, rHeight) + 1
}

func max(a, b int) int {
    if a > b {
        return a
    }
    return b
}

func main() {
    root := &TreeNode{Val: 3}
    node1 := &TreeNode{Val: 9}
    node2 := &TreeNode{Val: 20}
    node3 := &TreeNode{Val: 15}
    node4 := &TreeNode{Val: 7}
    root.Left = node1
    root.Right = node2
    node2.Left = node3
    node2.Right = node4

    depth := maxDepth(root)
    fmt.Println(depth)
}
