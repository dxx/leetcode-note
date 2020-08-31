package main

import (
    "fmt"
    "math"
)

// 验证二叉搜索树
// 给定一个二叉树，判断其是否是一个有效的二叉搜索树。
// 假设一个二叉搜索树具有如下特征：
// 节点的左子树只包含小于当前节点的数。
// 节点的右子树只包含大于当前节点的数。
// 所有左子树和右子树自身必须也是二叉搜索树。

// 示例
// 输入: [2,1,3]
//         2
//        / \
//       1   3
// 输出: true

// 输入: [5,1,4,null,null,3,6]
//         5
//        / \
//       1   4
//          / \
//         3   6
// 输出: false
// 解释: 根节点的值为 5 ，但是其右子节点值为 4 。

type TreeNode struct {
    Val int
    Left *TreeNode
    Right *TreeNode
}

func isValidBST(root *TreeNode) bool {
    return compare(root, math.MinInt64, math.MaxInt64)
}

func compare(node *TreeNode, lower, upper int) bool {
    if node == nil {
        return true
    }
    // 当前节点小于最小值或者大于最大值
    if node.Val <= lower || node.Val >= upper {
        return false
    }
    // 左子树区间为最小值-当前节点的值, 右子树区间为当前节点的值-最大值
    return compare(node.Left, lower, node.Val) && compare(node.Right, node.Val, upper)
}

func preOrder(node *TreeNode) {
    if node == nil {
        return
    }
    fmt.Println(node.Val)
    preOrder(node.Left)
    preOrder(node.Right)
}

func main() {
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
