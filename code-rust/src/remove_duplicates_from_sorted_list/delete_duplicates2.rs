/// 删除排序链表中的重复元素 II
/// 给定一个排序链表，删除所有含有重复数字的节点，只保留原始链表中 没有重复出现 的数字。

/// 示例
/// 输入: 1->2->3->3->4->4->5
/// 输出: 1->2->5

/// 输入: 1->1->1->2->3
/// 输出: 2->3

#[derive(PartialEq, Eq, Clone, code_rust_macro::ListNodeDebug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

/*impl std::fmt::Debug for ListNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut node = &mut self.clone();
        let mut str = "".to_string();
        loop {
            str.push_str(&node.val.to_string());
            str.push_str("->");
            node = match node.next.as_mut() {
                Some(n) => n.as_mut(),
                None => break,
            };
        }
        str.remove(str.len() - 1);
        str.remove(str.len() - 1);
        write!(f, "{}", str)
    }
}*/

pub struct Solution {}

impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut new_head = Some(Box::new(ListNode::new(-1)));
        new_head.as_mut().unwrap().next = head;
        let mut current = &mut new_head;
        while current.is_some() && current.as_ref().unwrap().next.is_some() {
            let next = &mut current.as_mut().unwrap().next;
            // 如果到最后一个节点或者下一个节点和下下一个节点不同
            if next.as_ref().unwrap().next.is_none()
                || next.as_ref().unwrap().next.as_ref().unwrap().val != next.as_ref().unwrap().val
            {
                current = &mut current.as_mut().unwrap().next;
                continue;
            }

            let replace_node = Solution::get_different_node(next);
            // 跳过相同的节点, 指向下一个不同的节点
            if let Some(node) = replace_node {
                // 当前节点替换掉下一个节点
                next.replace(node);
            } else {
                // 拿走当前节点保留 None
                next.take();
            }
        }
        new_head.take().unwrap().next
    }

    fn get_different_node(node: &mut Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut temp = node;
        // 下一个节点相同, 一直遍历直到不相同的节点
        while temp.as_ref().unwrap().next.is_some()
            && temp.as_ref().unwrap().next.as_ref().unwrap().val == temp.as_ref().unwrap().val
        {
            temp = &mut temp.as_mut().unwrap().next;
        }
        temp.take().unwrap().next
    }
}

#[test]
fn test_delete_duplicates() {
    let mut head = Some(Box::new(ListNode::new(1)));
    let mut node2 = Some(Box::new(ListNode::new(2)));
    let mut node3 = Some(Box::new(ListNode::new(3)));
    let mut node4 = Some(Box::new(ListNode::new(3)));
    let mut node5 = Some(Box::new(ListNode::new(4)));
    let mut node6 = Some(Box::new(ListNode::new(4)));
    let node7 = Some(Box::new(ListNode::new(5)));

    node6.as_mut().unwrap().next = node7;
    node5.as_mut().unwrap().next = node6;
    node4.as_mut().unwrap().next = node5;
    node3.as_mut().unwrap().next = node4;
    node2.as_mut().unwrap().next = node3;
    head.as_mut().unwrap().next = node2;

    println!("{:?}", head);

    let head = Solution::delete_duplicates(head);

    println!("{:?}", head);

    let mut head = Some(Box::new(ListNode::new(1)));
    let mut node2 = Some(Box::new(ListNode::new(1)));
    let mut node3 = Some(Box::new(ListNode::new(1)));
    let mut node4 = Some(Box::new(ListNode::new(2)));
    let node5 = Some(Box::new(ListNode::new(3)));

    node4.as_mut().unwrap().next = node5;
    node3.as_mut().unwrap().next = node4;
    node2.as_mut().unwrap().next = node3;
    head.as_mut().unwrap().next = node2;

    println!("{:?}", head);

    let head = Solution::delete_duplicates(head);

    println!("{:?}", head);
}
