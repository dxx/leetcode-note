package main

import "fmt"

// 相同的树
// 给定两个二叉树，编写一个函数来检验它们是否相同。
// 如果两个树在结构上相同，并且节点具有相同的值，则认为它们是相同的。

// 示例
// 输入:   1         1
//       / \       / \
//      2   3     2   3
//
//      [1,2,3],   [1,2,3]
//
// 输出: true

// 输入:    1          1
//        /            \
//       2              2
//
//      [1,2],     [1,null,2]
//
// 输出: false

// 输入:    1         1
//        / \       / \
//       2   1     1   2
//
//      [1,2,1],   [1,1,2]
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

func main() {
    p := &TreeNode{Val: 1}
    p2 := &TreeNode{Val: 2}
    p3 := &TreeNode{Val: 3}
    p.Left = p2
    p.Right = p3

    q := &TreeNode{Val: 1}
    q2 := &TreeNode{Val: 2}
    q3 := &TreeNode{Val: 3}
    q.Left = q2
    q.Right = q3

    fmt.Print("p: ")
    infixOrder(p)
    fmt.Println()

    fmt.Print("q: ")
    infixOrder(q)
    fmt.Println()

    isSame := isSameTree(p, q)
    fmt.Println(isSame)


    p = &TreeNode{Val: 1}
    p2 = &TreeNode{Val: 2}
    p.Left = p2

    q = &TreeNode{Val: 1}
    q2 = &TreeNode{Val: 2}
    q.Right = q2

    fmt.Print("p: ")
    infixOrder(p)
    fmt.Println()

    fmt.Print("q: ")
    infixOrder(q)
    fmt.Println()

    isSame = isSameTree(p, q)
    fmt.Println(isSame)


    p = &TreeNode{Val: 1}
    p2 = &TreeNode{Val: 2}
    p3 = &TreeNode{Val: 1}
    p.Left = p2
    p.Right = p3

    q = &TreeNode{Val: 1}
    q2 = &TreeNode{Val: 1}
    q3 = &TreeNode{Val: 2}
    q.Left = q2
    q.Right = q3

    fmt.Print("p: ")
    infixOrder(p)
    fmt.Println()

    fmt.Print("q: ")
    infixOrder(q)
    fmt.Println()

    isSame = isSameTree(p, q)
    fmt.Println(isSame)
}
