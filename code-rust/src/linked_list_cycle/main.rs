/// 环形链表
/// 给定一个链表，判断链表中是否有环。
/// 为了表示给定链表中的环，我们使用整数 pos 来表示链表尾连接到链表中的位置（索引从 0 开始）。
/// 如果 pos 是 -1，则在该链表中没有环。

/// 进阶: 你能用 O(1)（即常量）内存解决此问题吗？

/// 示例
/// 输入: head = [3,2,0,-4], pos = 1
/// 输出: true
/// 解释: 链表中有一个环，其尾部连接到第二个节点。

/// 输入: head = [1,2], pos = 0
/// 输出: true
/// 解释: 链表中有一个环，其尾部连接到第一个节点。

/// 输入: head = [1], pos = -1
/// 输出: false
/// 解释: 链表中没有环。

use std::rc::Rc;
use std::cell::RefCell;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Rc<RefCell<ListNode>>>
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub struct Solution {}

impl Solution {
    pub fn has_cycle(head: Option<Rc<RefCell<ListNode>>>) -> bool {
        if head.is_none() {
            return false;
        }
        let mut fast = head.unwrap();
        let mut slow = fast.clone();

        loop {
            // 快指针走两个节点
            for _ in 0..2 {
                if fast.borrow().next.is_none() {
                    return false;
                }
                let fast_next = fast.borrow().next.clone().unwrap();
                fast = fast_next;
            }
            // 慢指针走一个节点
            let slow_next = slow.borrow().next.clone().unwrap();
            slow = slow_next;
            // 如果有环, 快指针一定会追上慢指针
            if fast.eq(&slow) {
                return true;
            }
        }
    }
}

#[test]
fn test_has_cycle() {
    let head = Rc::new(RefCell::new(ListNode::new(3)));
    let node2 = Rc::new(RefCell::new(ListNode::new(2)));
    let node3 = Rc::new(RefCell::new(ListNode::new(0)));
    let node4 = Rc::new(RefCell::new(ListNode::new(-1)));

    node4.borrow_mut().next = Some(Rc::clone(&node2));
    node3.borrow_mut().next = Some(node4);
    node2.borrow_mut().next = Some(node3);
    head.borrow_mut().next = Some(Rc::clone(&node2));

    let result = Solution::has_cycle(Some(head));
    println!("{}", result);

    let head = Rc::new(RefCell::new(ListNode::new(1)));
    let node2 = Rc::new(RefCell::new(ListNode::new(2)));

    node2.borrow_mut().next = Some(Rc::clone(&head));
    head.borrow_mut().next = Some(Rc::clone(&node2));

    let result = Solution::has_cycle(Some(head));
    println!("{}", result);

    let head = Rc::new(RefCell::new(ListNode::new(1)));

    let result = Solution::has_cycle(Some(head));
    println!("{}", result);
}
