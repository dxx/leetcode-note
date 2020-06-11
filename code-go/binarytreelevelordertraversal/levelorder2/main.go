package levelorder2

// 二叉树的层次遍历 II
// 给定一个二叉树，返回其节点值自底向上的层次遍历。 （即按从叶子节点所在层到根节点所在的层，逐层从左向右遍历）

// 示例
// 输入: [3,9,20,null,null,15,7]
//        3
//       / \
//      9  20
//        /  \
//       15   7
// 输出: [
//        [15,7],
//        [9,20],
//        [3]
//      ]

type TreeNode struct {
    Val int
    Left *TreeNode
    Right *TreeNode
}

func levelOrderBottom(root *TreeNode) [][]int {
    if root == nil {
        return  nil
    }
    // 当做队列
    q := make([]*TreeNode, 0)
    // 根节点先入队列
    q = append(q, root)
    results := make([][]int, 0)
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

        temp := [][]int{ val }
        // 将当前所有的结果添加到最前面
        results = append(temp, results...)

        // 修改成下一次遍历的队列
        q = p
    }
    return results
}
