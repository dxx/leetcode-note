package sametree

import "fmt"

// 相同的树
// 给定两个二叉树，编写一个函数来检验它们是否相同。
// 如果两个树在结构上相同，并且节点具有相同的值，则认为它们是相同的。

// 示例
// 输入: [1,2,3],  [1,2,3]
//         1         1
//        / \       / \
//       2   3     2   3
//
// 输出: true

// 输入: [1,2],    [1,null,2]
//         1          1
//        /            \
//       2              2
//
// 输出: false

// 输入: [1,2,1],  [1,1,2]
//         1         1
//        / \       / \
//       2   1     1   2
//
// 输出: false

type TreeNode struct {
    Val int
    Left *TreeNode
    Right *TreeNode
}

func isSameTree(p *TreeNode, q *TreeNode) bool {
    if p == nil && q == nil {
        return true
    }
    if p == nil || q == nil {
        return false
    }
    // 比较左子树
    isSame := isSameTree(p.Left, q.Left)
    // 不相等直接返回
    if !isSame {
        return false
    }
    // 中序比较当前节点值是否相等
    if p.Val != q.Val {
        return false
    }
    // 比较右子树
    return isSameTree(p.Right, q.Right)
}

func infixOrder(node *TreeNode)  {
    if node == nil {
        return
    }
    infixOrder(node.Left)
    fmt.Print(node.Val)
    infixOrder(node.Right)
}
