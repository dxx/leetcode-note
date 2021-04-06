/// 排序链表
/// 在 O(n log n) 时间复杂度和常数级空间复杂度下，对链表进行排序。

/// 示例
/// 输入: 4->2->1->3
/// 输出: 1->2->3->4

/// 输入: -1->5->3->4->0
/// 输出: -1->0->3->4->5

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
    pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let len = Solution::get_len(&head);
        Solution::sort(head, len)
    }

    /// 分解排序
    fn sort(mut head: Option<Box<ListNode>>, len: usize) -> Option<Box<ListNode>> {
        if head.is_none() || head.as_ref().unwrap().next.is_none() {
            return head;
        }
        let middle = len >> 1;
        let mut middle_node = &mut head;
        for _ in 0..middle {
            middle_node = &mut middle_node.as_mut().unwrap().next;
        }
        // 断开链表
        let middle_node = middle_node.take();

        // 分解左边
        let head1 = Solution::sort(head, middle);
        // 分解右边
        let head2 = Solution::sort(middle_node, len - middle);
        // 将两个有序的链表合并成一个链表
        Solution::merge(head1, head2)
    }

    /// 合并有序链表
    fn merge(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut new_list_node = Some(Box::new(ListNode::new(0)));
        let mut prev_new_node = &mut new_list_node;
        let (mut node1, mut node2) = (&l1, &l2);
        while node1.is_some() && node2.is_some() {
            let node_box1 = node1.as_ref().unwrap();
            let node_box2 = node2.as_ref().unwrap();
            // 判断左边链表节点是否小于等于右边链表节点
            if node_box1.val <= node_box2.val {
                // 将下一个节点添加到上一个节点的后面
                prev_new_node.as_mut().unwrap().next =
                    Some(Box::new(ListNode::new(node_box1.val)));
                // 左边当前链表节点后移
                node1 = &node_box1.next;
            } else {
                // 判断左边链表节点是大于右边链表节点
                // 将下一个节点添加到上一个节点的后面
                prev_new_node.as_mut().unwrap().next =
                    Some(Box::new(ListNode::new(node_box2.val)));
                // 右边当前链表节点后移
                node2 = &node_box2.next;
            }
            // 记录上一个节点
            prev_new_node = &mut prev_new_node.as_mut().unwrap().next;
        }

        // 左边链表还有剩余节点，直接添加到末尾
        if node1.is_some() {
            prev_new_node.as_mut().unwrap().next = node1.clone()
        }
        // 右边链表还有剩余节点，直接添加到末尾
        if node2.is_some() {
            prev_new_node.as_mut().unwrap().next = node2.clone();
        }

        new_list_node.take().unwrap().next
    }

    fn get_len(head: &Option<Box<ListNode>>) -> usize {
        let mut len = 0;
        let mut current = head;
        while let Some(node) = current.as_ref() {
            current = &node.next;
            len += 1
        }
        len
    }
}

#[test]
fn test_sort_list() {
    let mut node1 = Some(Box::new(ListNode::new(4)));
    let mut node2 = Some(Box::new(ListNode::new(2)));
    let mut node3 = Some(Box::new(ListNode::new(1)));
    let node4 = Some(Box::new(ListNode::new(3)));

    node3.as_mut().unwrap().next = node4;
    node2.as_mut().unwrap().next = node3;
    node1.as_mut().unwrap().next = node2;

    println!("{:?}", node1);

    let node = Solution::sort_list(node1);

    println!("{:?}", node);

    let mut node1 = Some(Box::new(ListNode::new(-1)));
    let mut node2 = Some(Box::new(ListNode::new(5)));
    let mut node3 = Some(Box::new(ListNode::new(3)));
    let mut node4 = Some(Box::new(ListNode::new(4)));
    let node5 = Some(Box::new(ListNode::new(0)));

    node4.as_mut().unwrap().next = node5;
    node3.as_mut().unwrap().next = node4;
    node2.as_mut().unwrap().next = node3;
    node1.as_mut().unwrap().next = node2;

    println!("{:?}", node1);

    let node = Solution::sort_list(node1);

    println!("{:?}", node);
}
