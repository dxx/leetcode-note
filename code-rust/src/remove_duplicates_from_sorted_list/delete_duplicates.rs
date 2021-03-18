/// 删除排序链表中的重复元素
/// 给定一个排序链表，删除所有重复的元素，使得每个元素只出现一次。

/// 示例
/// 输入: 1->1->2
/// 输出: 1->2

/// 输入: 1->1->2->3->3
/// 输出: 1->2->3

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
        let mut head = head;
        let mut current = &mut head;
        while let Some(current_node) = current {
            // 找到下一个不相同的节点
            while current_node.next.is_some()
                && current_node.val == current_node.next.as_ref().unwrap().val
            {
                // 下下一个节点
                let replace_node = current_node.next.as_mut().unwrap().next.take();
                if let Some(node) = replace_node {
                    // 当前节点的下一个节点替换掉下下一个节点
                    current_node.next.replace(node);
                } else {
                    // 下下一个节点为 None 时，
                    current_node.next = None;
                }
            }
            current = &mut current_node.next;
        }
        head
    }
}

#[test]
fn test_delete_duplicates() {
    let mut head = Some(Box::new(ListNode::new(1)));
    let mut node2 = Some(Box::new(ListNode::new(1)));
    let node3 = Some(Box::new(ListNode::new(2)));

    node2.as_mut().unwrap().next = node3;
    head.as_mut().unwrap().next = node2;

    println!("{:?}", head);

    let head = Solution::delete_duplicates(head);

    println!("{:?}", head);

    let mut head = Some(Box::new(ListNode::new(1)));
    let mut node2 = Some(Box::new(ListNode::new(1)));
    let mut node3 = Some(Box::new(ListNode::new(2)));
    let mut node4 = Some(Box::new(ListNode::new(3)));
    let node5 = Some(Box::new(ListNode::new(3)));

    node4.as_mut().unwrap().next = node5;
    node3.as_mut().unwrap().next = node4;
    node2.as_mut().unwrap().next = node3;
    head.as_mut().unwrap().next = node2;

    println!("{:?}", head);

    let head = Solution::delete_duplicates(head);

    println!("{:?}", head);
}
