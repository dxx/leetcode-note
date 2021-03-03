/// 两两交换链表中的节点
/// 给定一个链表，两两交换其中相邻的节点，并返回交换后的链表。**你不能只是单纯的改变节点内部的值**，而是需要实际的进行节点交换。

/// 示例
/// 输入: 1->2->3->4
/// 输出: 2->1->4->3

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
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut swap_head_node = ListNode::new(-1);
        let mut prev = &mut swap_head_node.next;
        let mut temp_node = head;
        while let Some(mut current) = temp_node.take() {
            if let Some(mut next) = current.next.take() {
                // 当前节点的下一个节点的下一个节点指向上一个节点，并返回下一个节点给 temp_node
                temp_node = next.next.replace(current);
                // 指向下一个节点
                prev.replace(next);
                // 指向交换后的下一个节点
                prev = &mut prev.as_mut().unwrap().next.as_mut().unwrap().next;
            } else {
                // 替换当前节点
                prev.replace(current);
            }
        }
        swap_head_node.next
    }
}

#[test]
fn test_swap_pairs() {
    let mut head = Some(Box::new(ListNode::new(1)));
    let mut node1 = Some(Box::new(ListNode::new(2)));
    let mut node2 = Some(Box::new(ListNode::new(3)));
    let node3 = Some(Box::new(ListNode::new(4)));

    node2.as_mut().unwrap().next = node3;
    node1.as_mut().unwrap().next = node2;
    head.as_mut().unwrap().next = node1;

    println!("交换前：");
    println!("{:?}", head);

    let new_node = Solution::swap_pairs(head);

    println!("交换后：");
    println!("{:?}", new_node);
}