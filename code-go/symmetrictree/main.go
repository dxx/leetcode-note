package symmetrictree

// 对称二叉树
// 给定一个二叉树，检查它是否是镜像对称的。如果一个树的左子树与右子树结构相同, 对应的节点值也相同，那么这个树是镜像对称的。

// 示例
// 输入: [1,2,2,3,4,4,3]
//          1
//         / \
//        2   2
//       / \ / \
//      3  4 4  3
// 输出: true

// 输入: [1,2,2,null,3,null,3]
//         1
//        / \
//       2   2
//        \   \
//        3    3
// 输出: false

type TreeNode struct {
    Val int
    Left *TreeNode
    Right *TreeNode
}

func isSymmetric(root *TreeNode) bool {
    return checkSymmetrical(root, root)
}

func checkSymmetrical(node1, node2 *TreeNode) bool {
    if node1 == nil && node2 == nil {
        return true
    }
    // 只要有一个节点为 nil, 表示镜像非对称
    if node1 == nil || node2 == nil {
        return false
    }
    // 节点不相同表示非镜像对称
    if node1.Val != node2.Val {
        return false
    }
    // 比较两棵树的左右子树是否是镜像对称的
    return checkSymmetrical(node1.Left, node2.Right) && checkSymmetrical(node1.Right, node2.Left)
}
