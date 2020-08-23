package main

import "fmt"

// 将有序数组转换为二叉搜索树
// 将一个按照升序排列的有序数组，转换为一棵高度平衡二叉搜索树。
// 本题中，一个高度平衡二叉树是指一个二叉树每个节点 的左右两个子树的高度差的绝对值不超过 1。

// 示例
// 输入: [-10, -3, 0, 5, 9]
// 输出:      0
//          / \
//        -3   9
//        /   /
//      -10  5
//      [0, -3, 9, -10, null, 5]

type TreeNode struct {
    Val int
    Left *TreeNode
    Right *TreeNode
}

func sortedArrayToBST(nums []int) *TreeNode {
    return buildTree(nums, 0, len(nums) - 1)
}

func buildTree(nums []int, start, end int) *TreeNode {
    if start > end {
        return nil
    }
    // (start + end) >> 1 也可以是一个中间下标
    // 选择中间位置的右边一个节点作为根节点
    mid := (start + end + 1) >> 1
    // 数组的中间节点作为树的根节点
    root := &TreeNode{Val: nums[mid]}
    // 构建左子树
    root.Left = buildTree(nums, start, mid - 1)
    // 构建右子树
    root.Right = buildTree(nums, mid + 1, end)
    return root
}

func infixOrder(node *TreeNode) {
    if node == nil {
        return
    }
    infixOrder(node.Left)
    fmt.Println(node.Val)
    infixOrder(node.Right)
}

func main() {
    nums := []int{-10, -3, 0, 5, 9}

    fmt.Println(nums)

    tree := sortedArrayToBST(nums)

    infixOrder(tree)
}
