package minimumdepthofbinarytree

// 二叉树的最小深度
// 给定一个二叉树，找出其最小深度。
// 最小深度是从根节点到最近叶子节点的最短路径上的节点数量。
// 说明: 叶子节点是指没有子节点的节点。

// 示例
// 给定二叉树 [3,9,20,null,null,15,7]
//   3
//  / \
// 9  20
//   /  \
//  15   7
// 它的最小深度是 2 。

type TreeNode struct {
    Val int
    Left *TreeNode
    Right *TreeNode
}

func minDepth(root *TreeNode) int {
    if root == nil {
        return 0
    }
    // 左子树的高度
    lHeight := minDepth(root.Left)
    // 右子树的高度
    rHeight := minDepth(root.Right)
    // 左子节点为空，返回右子节点的深度
    if root.Left == nil {
        return rHeight + 1
    }
    // 右子节点为空，返回左子节点的深度
    if root.Right == nil {
        return lHeight + 1
    }
    // 返回最小的高度, 当前节点的高度为 1
    return min(lHeight, rHeight) + 1
}

func min(a, b int) int {
    if a < b {
        return a
    }
    return b
}
