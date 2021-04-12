/// 反转链表
/// 反转一个单链表。

/// 示例
/// 输入: 1->2->3->4->5
/// 输出: 5->4->3->2->1

#[derive(PartialEq, Eq, Clone, code_rust_macro::ListNodeDebug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub struct Solution {}

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut reverse_node = None;
        let mut current = head;
        while let Some(mut node) = current {
            // 记录下一个节点
            let next = node.next.take();
            // 将当前节点指向翻转后的第一个节点
            node.next = reverse_node;
            // 修改新的头节点的下一个节点
            reverse_node = Some(node);

            // 指向下一个节点
            current = next;
        }
        reverse_node
    }
}

#[test]
fn test_reverse_list() {
    let mut head = Some(Box::new(ListNode::new(1)));
    let mut node2 = Some(Box::new(ListNode::new(2)));
    let mut node3 = Some(Box::new(ListNode::new(3)));
    let mut node4 = Some(Box::new(ListNode::new(4)));
    let node5 = Some(Box::new(ListNode::new(5)));

    node4.as_mut().unwrap().next = node5;
    node3.as_mut().unwrap().next = node4;
    node2.as_mut().unwrap().next = node3;
    head.as_mut().unwrap().next = node2;

    println!("{:?}", head);

    let head = Solution::reverse_list(head);

    println!("{:?}", head);
}
