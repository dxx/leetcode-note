package uniquebinarysearchtree

import "fmt"

// 不同的二叉搜索树 II
// 给定一个整数 n，生成所有由 1 ... n 为节点所组成的 二叉搜索树 。

// 示例
// 输入: 3
// 输出:
// [
//  [1,null,3,2],
//  [3,2,null,1],
//  [3,1,null,null,2],
//  [2,1,3],
//  [1,null,2,null,3]
// ]
// 解释：
// 以上的输出对应以下 5 种不同结构的二叉搜索树：
//   1         3     3      2      1
//    \       /     /      / \      \
//     3     2     1      1   3      2
//    /     /       \                 \
//   2     1         2                 3

type TreeNode struct {
    Val int
    Left *TreeNode
    Right *TreeNode
}

func preOrder(node *TreeNode) {
    if node == nil {
        return
    }
    fmt.Printf("%2d",node.Val)
    preOrder(node.Left)
    preOrder(node.Right)
}

func generateTrees(n int) []*TreeNode {
    if n == 0 {
        return nil
    }
    return generateSubTrees(1, n)
}

func generateSubTrees(start, end int) []*TreeNode {
    if start > end {
        // 如果没有节点，返回 nil
        return []*TreeNode{nil}
    }
    var treeNodes []*TreeNode
    for i := start; i <= end; i++ {
        // 获取所有可能的左子节点，由于递归左子树已经生成
        leftNodes := generateSubTrees(start, i - 1)
        // 获取所有可能的右子节点，由于递归右子树已经生成
        rightNodes := generateSubTrees(i + 1, end)
        for _, leftNode := range leftNodes {
            for _, rightNode := range rightNodes {
                root := &TreeNode{Val: i}
                // 选取一个节点作为左子节点
                root.Left = leftNode
                // 选取一个节点作为右子节点
                root.Right = rightNode
                treeNodes = append(treeNodes, root)
            }
        }
    }
    return treeNodes
}
