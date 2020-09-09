package main

import "fmt"

// 二叉树的后序遍历
// 给定一个二叉树，返回它的后序遍历。

// 进阶: 递归算法很简单，你可以通过迭代算法完成吗？

// 示例
// 输入: [1,null,2,3]
//   1
//    \
//     2
//    /
//   3
// 输出: [3,2,1]

type TreeNode struct {
    Val int
    Left *TreeNode
    Right *TreeNode
}

func postorderTraversal(root *TreeNode) []int {
    // 非递归
    return postorderNoRecursion(root)
    // 递归
    // return postorderRecursion(root)
}

// 非递归后序遍历
func postorderNoRecursion(node *TreeNode) []int {
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

        out := make([]int, 1, len(output) + 1)
        out[0] = node.Val
        // 后访问的节点添加到最前面
        output = append(out, output...)

        if node.Left != nil {
            // 先将左子节点压入栈, 先入栈的后出栈
            stack = append(stack, node.Left)
        }
        if node.Right != nil {
            // 将右子节点压入栈, 后入栈的先出栈
            stack = append(stack, node.Right)
        }

    }
    return output
}

// 递归后序遍历
func postorderRecursion(node *TreeNode) []int {
    if node == nil {
        return []int{}
    }
    nodes := make([]int, 0)
    // 先访问左子节点
    nodes = append(nodes, postorderRecursion(node.Left)...)
    // 再访问右子节点
    nodes = append(nodes, postorderRecursion(node.Right)...)
    // 最后将当前节点存入数组
    nodes = append(nodes, node.Val)
    return nodes
}

func main() {
    node1 := &TreeNode{Val: 1}
    node2 := &TreeNode{Val: 2}
    node3 := &TreeNode{Val: 3}
    node1.Right = node2
    node2.Left = node3

    results := postorderTraversal(node1)

    fmt.Println(results)
}
