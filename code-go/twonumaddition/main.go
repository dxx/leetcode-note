package twonumaddition

import (
    "fmt"
    "strings"
)

// 两数相加
// 给出两个 **非空** 的链表用来表示两个非负的整数。其中，它们
// 各自的位数是按照 **逆序** 的方式存储的，并且它们的每个节点
// 只能存储 **一位** 数字。
// 如果，我们将这两个数相加起来，则会返回一个新的链表来表示它们的和。
// 您可以假设除了数字 0 之外，这两个数都不会以 0 开头。

// 示例
// 输入: (2 -> 4 -> 3) + (5 -> 6 -> 4)
// 输出: 7 -> 0 -> 8
// 原因: 342 + 465 = 807

// 输入: (2 -> 4 -> 3) + (5 -> 6)
// 输出: 7 -> 0 -> 4
// 原因: 342 + 65 = 407

// 输入: (2 -> 4) + (5 -> 6)
// 输出: 7 -> 0 -> 1
// 原因: 42 + 65 = 107

// 单链表结构体
type NumNode struct {
    Val int
    Next *NumNode
}

func (numNode *NumNode) String() string {
    node := numNode
    var str = ""
    for node != nil {
        str += fmt.Sprintf("%d->", node.Val)
        node = node.Next
    }
    str = strings.TrimSuffix(str, "->")
    return str
}

func addTwoNumbers(l1 *NumNode, l2 *NumNode) *NumNode {
    var newNumNode = &NumNode{Val: 0}
    var prevNode = newNumNode
    var node1, node2 = l1, l2
    var val, carry int

    for node1 != nil || node2 != nil {
        val1 := 0
        val2 := 0
        // 第一个链表节点不为空继续往下一个节点走
        if node1 != nil {
            val1 = node1.Val
            node1 = node1.Next
        }
        // 第二个链表节点不为空继续往下一个节点走
        if node2 != nil {
            val2 = node2.Val
            node2 = node2.Next
        }
        // 两个节点的值加上上一次相加的进位
        val = val1 + val2 + carry
        // 获取进位
        carry = val / 10
        // 创建新的节点
        newNode := &NumNode{Val: val % 10}
        // 将新节点添加到上一个节点末尾
        prevNode.Next = newNode
        // 记录上一个节点
        prevNode = newNode
    }

    // 如果有进位增加一个新的节点
    if carry > 0 {
        prevNode.Next = &NumNode{Val: carry}
    }

    return newNumNode.Next
}
