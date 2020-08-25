package main

import "fmt"

// 二叉树的层序遍历
// 给你一个二叉树，请你返回其按 **层序遍历** 得到的节点值。 （即逐层地，从左到右访问所有节点）。

// 示例
// 输入: [3,9,20,null,null,15,7]
//        3
//       / \
//      9  20
//        /  \
//       15   7
// 输出: [
//        [3],
//        [9,20],
//        [15,7]
//      ]

type TreeNode struct {
    Val int
    Left *TreeNode
    Right *TreeNode
}

func levelOrder(root *TreeNode) [][]int {
    return bfs(root)

    //results := make([][]int, 0)
    //results = dfs(root, 1, results)
    //return results
}

// 广度优先
func bfs (root *TreeNode) [][]int {
    results := make([][]int, 0)
    if root == nil {
        return results
    }
    // 当做队列
    q := make([]*TreeNode, 0)
    // 根节点先入队列
    q = append(q, root)
    for len(q) > 0 {
        val := make([]int, 0)
        // 临时存放下一级节点
        p := make([]*TreeNode, 0)
        // 遍历当前层级的所有节点
        for i := 0; i < len(q); i++ {
            // 当前层级的根节点
            node := q[i]
            val = append(val, node.Val)
            if node.Left != nil {
                p = append(p, node.Left)
            }
            if node.Right != nil {
                p = append(p, node.Right)
            }
        }
        results = append(results, val)
        // 修改成下一次遍历的队列
        q = p
    }
    return results
}

// 深度优先
func dfs(node *TreeNode, level int, results [][]int) [][]int {
    if node == nil {
        return [][]int{}
    }
    if level - 1 >= len(results) {
        val := make([]int, 0)
        results = append(results, val)
    }
    // results 本身的引用会被修改
    results[level - 1] = append(results[level - 1], node.Val)

    // 递归左子节点
    leftResults := dfs(node.Left, level + 1, results)
    if len(leftResults) > 0 {
        // 重新修改结果集
        results = leftResults
    }
    // 递归右子节点
    rightResults := dfs(node.Right, level + 1, results)
    if len(rightResults) > 0 {
        results = rightResults
    }
    return results
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

    results := levelOrder(root)
    fmt.Println(results)
}
