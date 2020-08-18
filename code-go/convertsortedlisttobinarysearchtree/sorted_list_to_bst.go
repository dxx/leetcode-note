package main

import "fmt"

// 有序链表转换二叉搜索树
// 给定一个单链表，其中的元素按升序排序，将其转换为高度平衡的二叉搜索树。
// 本题中，一个高度平衡二叉树是指一个二叉树每个节点 的左右两个子树的高度差的绝对值不超过 1。

// 示例
// 输入: [-10, -3, 0, 5, 9]
// 输出:      0
//          / \
//        -3   9
//        /   /
//      -10  5
//      [0, -3, 9, -10, null, 5]

type ListNode struct {
    Val int
    Next *ListNode
}

type TreeNode struct {
    Val int
    Left *TreeNode
    Right *TreeNode
}

func sortedListToBST(head *ListNode) *TreeNode {
    // return buildTree(head, nil)
    length := getLength(head)
    gCurrent = head
    return buildTree2(0, length - 1)
}

// 构建平衡二叉树
func buildTree(head *ListNode, tail *ListNode) *TreeNode {
    if head == tail {
        return nil
    }
    middleNode := getMiddleNode(head, tail)
    // 链表的中间节点作为树的根节点
    root := &TreeNode{Val: middleNode.Val}
    // 构建左子树
    root.Left = buildTree(head, middleNode)
    // 构建右子树
    root.Right = buildTree(middleNode.Next, tail)
    return root
}

// 获取中间节点
func getMiddleNode(head *ListNode, tail *ListNode) *ListNode {
    fast, slow := head, head
    for fast != tail && fast.Next != tail {
        // 快指针走两个节点
        fast = fast.Next
        fast = fast.Next
        // 慢指针走一个节点
        slow = slow.Next
    }
    return slow
}

var gCurrent *ListNode
// 分治+中序遍历构建二叉树
func buildTree2(start, end int) *TreeNode {
    if start > end {
        return nil
    }
    root := &TreeNode{}
    // 中间下标，用来分解
    // (start + end) >> 1 也可以是一个中间下标
    mid := (start + end + 1) >> 1
    // 构建左子树
    root.Left = buildTree2(start, mid - 1)
    // 中序遍历，等同于链表遍历
    root.Val = gCurrent.Val
    gCurrent = gCurrent.Next
    // 构建右子树
    root.Right = buildTree2(mid + 1, end)
    return root
}

func getLength(head *ListNode) int {
    l := 0
    current := head
    for current != nil {
        current = current.Next
        l++
    }
    return l
}

func infixOrder(node *TreeNode) {
    if node == nil {
        return
    }
    infixOrder(node.Left)
    fmt.Println(node.Val)
    infixOrder(node.Right)
}

func main() {
    node1 := &ListNode{Val: -10}
    node2 := &ListNode{Val: -3}
    node3 := &ListNode{Val: 0}
    node4 := &ListNode{Val: 5}
    node5 := &ListNode{Val: 9}
    node1.Next = node2
    node2.Next = node3
    node3.Next = node4
    node4.Next = node5

    tree := sortedListToBST(node1)

    infixOrder(tree)
}
