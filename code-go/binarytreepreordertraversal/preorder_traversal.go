package main

import "fmt"

// 二叉树的前序遍历
// 给定一个二叉树，返回它的前序遍历。

// 进阶: 递归算法很简单，你可以通过迭代算法完成吗？

// 示例
// 输入: [1,null,2,3]
//   1
//    \
//     2
//    /
//   3
// 输出: [1,2,3]

type TreeNode struct {
    Val int
    Left *TreeNode
    Right *TreeNode
}

func preorderTraversal(root *TreeNode) []int {
    // 非递归
    return preorderNoRecursion(root)
    // 递归
    // return preorderRecursion(root)
}

// 非递归前序遍历
func preorderNoRecursion(node *TreeNode) []int {
    var stack []*TreeNode
    output := make([]int, 0)
    if node == nil {
        return output
    }
    // 将当前节点压入栈
    stack = append(stack, node)
    for len(stack) > 0 {
        // 弹出当前节点
        node := stack[len(stack) - 1]
        stack = stack[:len(stack) - 1]
        output = append(output, node.Val)
        if node.Right != nil {
            // 先将右子节点压入栈, 先入栈的后出栈
            stack = append(stack, node.Right)
        }
        if node.Left != nil {
            // 将左子节点压入栈, 后入栈的先出栈
            stack = append(stack, node.Left)
        }
    }
    return output
}

// 递归前序遍历
func preorderRecursion(node *TreeNode) []int {
    if node == nil {
        return []int{}
    }
    nodes := make([]int, 0)
    // 先将当前节点存入数组
    nodes = append(nodes, node.Val)
    // 再访问左子节点
    nodes = append(nodes, preorderRecursion(node.Left)...)
    // 最后访问右子节点
    nodes = append(nodes, preorderRecursion(node.Right)...)
    return nodes
}

func main() {
    node1 := &TreeNode{Val: 1}
    node2 := &TreeNode{Val: 2}
    node3 := &TreeNode{Val: 3}
    node1.Right = node2
    node2.Left = node3

    results := preorderTraversal(node1)

    fmt.Println(results)
}
