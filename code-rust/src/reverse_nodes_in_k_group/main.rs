/// K 个一组翻转链表
/// 给你一个链表，每 k 个节点一组进行翻转，请你返回翻转后的链表。
/// k 是一个正整数，它的值小于或等于链表的长度。
/// 如果节点总数不是 k 的整数倍，那么请将最后剩余的节点保持原有顺序。

/// 示例
/// 输入: 1->2->3->4->5
/// 当 k = 2 时, 返回: 2->1->4->3->5
/// 当 k = 3 时, 返回: 3->2->1->4->5

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
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut tail = &mut head;
        // 寻找从 head 节点开始的第 k 个节点的下一个节点
        for _ in 0..k {
            if let Some(node) = tail.as_mut() {
                tail = &mut node.next;
            } else {
                return head;
            }
        }
        // 递归翻转
        let next_head = Solution::reverse_k_group(tail.take(), k);
        // 翻转 head 节点，连接 next_head
        Solution::reverse_connect(head, next_head)
    }

    fn reverse_connect(
        head: Option<Box<ListNode>>,
        next: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut temp = head;
        let mut last = next;
        while let Some(mut node) = temp {
            // 下一个节点
            let next = node.next.take();
            // 当前节点的下一个节点指向尾节点
            node.next = last.take();
            // 记录下一个节点
            last = Some(node);
            temp = next;
        }
        return last;
    }
}

#[test]
fn test_reverse_k_group() {
    let mut head = Some(Box::new(ListNode::new(1)));
    let mut node2 = Some(Box::new(ListNode::new(2)));
    let mut node3 = Some(Box::new(ListNode::new(3)));
    let mut node4 = Some(Box::new(ListNode::new(4)));
    let node5 = Some(Box::new(ListNode::new(5)));

    node4.as_mut().unwrap().next = node5;
    node3.as_mut().unwrap().next = node4;
    node2.as_mut().unwrap().next = node3.clone();
    head.as_mut().unwrap().next = node2;

    println!("{:?}", head);

    let k = 2;

    print!("k={}时, ", k);

    let head = Solution::reverse_k_group(head, k);

    println!("{:?}", head);

    let mut head = Some(Box::new(ListNode::new(1)));
    let mut node2 = Some(Box::new(ListNode::new(2)));
    let mut node3 = Some(Box::new(ListNode::new(3)));
    let mut node4 = Some(Box::new(ListNode::new(4)));
    let node5 = Some(Box::new(ListNode::new(5)));

    node4.as_mut().unwrap().next = node5;
    node3.as_mut().unwrap().next = node4;
    node2.as_mut().unwrap().next = node3;
    head.as_mut().unwrap().next = node2;

    let k = 3;

    print!("k={}时, ", k);

    let head = Solution::reverse_k_group(head, k);

    println!("{:?}", head);
}
