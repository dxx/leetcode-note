package sametree

import (
    "fmt"
    "testing"
)

func TestIsSameTree(t *testing.T) {
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
