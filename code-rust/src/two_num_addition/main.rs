/// 两数相加
/// 给出两个 **非空** 的链表用来表示两个非负的整数。其中，它们
/// 各自的位数是按照 **逆序** 的方式存储的，并且它们的每个节点
/// 只能存储 **一位** 数字。
/// 如果，我们将这两个数相加起来，则会返回一个新的链表来表示它们的和。
/// 您可以假设除了数字 0 之外，这两个数都不会以 0 开头。

/// 示例
/// 输入: (2 -> 4 -> 3) + (5 -> 6 -> 4)
/// 输出: 7 -> 0 -> 8
/// 原因: 342 + 465 = 807

/// 输入: (2 -> 4 -> 3) + (5 -> 6)
/// 输出: 7 -> 0 -> 4
/// 原因: 342 + 65 = 407

/// 输入: (2 -> 4) + (5 -> 6)
/// 输出: 7 -> 0 -> 1
/// 原因: 42 + 65 = 107

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
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut new_num_node = Box::new(ListNode::new(0));
        let mut prev_node = &mut new_num_node;
        let (mut node1, mut node2) = (&l1, &l2);
        let mut carry = 0;

        while node1.is_some() || node2.is_some() {
            let (mut val1, mut val2) = (0, 0);
            // 第一个链表节点不为空继续往下一个节点走
            if node1.is_some() {
                val1 = node1.as_ref().unwrap().val;
                node1 = &node1.as_ref().unwrap().next;
            }
            // 第二个链表节点不为空继续往下一个节点走
            if node2.is_some() {
                val2 = node2.as_ref().unwrap().val;
                node2 = &node2.as_ref().unwrap().next;
            }
            // 两个节点的值加上上一位数的进位
            let val = val1 + val2 + carry;
            carry = val / 10;
            // 将新节点添加到上一个节点末尾
            prev_node.next = Some(Box::new(ListNode::new(val % 10)));
            // 记录上一个节点
            prev_node = prev_node.next.as_mut().unwrap();
        }
        // 如果有进位增加一个新的节点
        if carry > 0 {
            prev_node.next = Some(Box::new(ListNode::new(carry)));
        }
        return new_num_node.next;
    }
}

#[test]
fn test_add_two_numbers() {
    let mut l1 = Some(Box::new(ListNode::new(2)));
    let mut node1 = Some(Box::new(ListNode::new(4)));
    let node2 = Some(Box::new(ListNode::new(3)));
    node1.as_mut().unwrap().next = node2;
    l1.as_mut().unwrap().next = node1;

    let mut l2 = Some(Box::new(ListNode::new(5)));
    let mut node3 = Some(Box::new(ListNode::new(6)));
    let node4 = Some(Box::new(ListNode::new(4)));
    node3.as_mut().unwrap().next = node4;
    l2.as_mut().unwrap().next = node3;

    println!("{:?}", l1);
    println!("{:?}", l2);

    let new_node = Solution::add_two_numbers(l1, l2);
    println!("{:?}", new_node);
}
