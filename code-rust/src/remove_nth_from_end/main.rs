/// 删除链表的倒数第N个节点
/// 给定一个链表，删除链表的倒数第 n 个节点，并且返回链表的头结点。

/// 说明:
/// 给定的 n 保证是有效的。

/// 示例
/// 输入: 1->2->3->4->5, n = 2
/// 输出: 1->2->3->5

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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        if head.is_none() || n <= 0 {
            return head;
        }
        // 创建一条相同的链表
        let mut new_head = Some(Box::new(ListNode::new(0)));
        new_head.as_mut().unwrap().next = head.clone();

        // 快指针，表示需要循环多少次
        let mut fast = &head;
        // 慢指针，表示倒数第 n 个节点，从要删除的上一个节点开始
        let mut slow = &mut new_head;
        let mut index = n;
        let mut len = 0;
        while fast.is_some() {
            len += 1;
            if index > 0 {
                // 先将快指针走到第 n 个节点
                fast = &fast.as_ref().unwrap().next;
                index -= 1;
                continue;
            }

            // 快慢指针同时走
            fast = &fast.as_ref().unwrap().next;
            slow = &mut slow.as_mut().unwrap().next;
        }
        if n <= len {
            // 删除节点
            slow.as_mut()?.next = slow.as_mut()?.next.take()?.next;
        }
        new_head?.next
    }
}

#[test]
fn test_remove_nth_from_end() {
    let mut head = Some(Box::new(ListNode::new(1)));
    let mut node2 = Some(Box::new(ListNode::new(2)));
    let mut node3 = Some(Box::new(ListNode::new(3)));
    let mut node4 = Some(Box::new(ListNode::new(4)));
    let node5 = Some(Box::new(ListNode::new(5)));

    node4.as_mut().unwrap().next = node5;
    node3.as_mut().unwrap().next = node4;
    node2.as_mut().unwrap().next = node3;
    head.as_mut().unwrap().next = node2;

    let n = 2;
    println!("{:?}", head);
    println!("n = {}", n);

    let head = Solution::remove_nth_from_end(head, n);

    println!("{:?}", head);
}
