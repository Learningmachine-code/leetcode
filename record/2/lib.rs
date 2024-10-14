pub mod listnode;
use crate::listnode::ListNode;
//Definition for singly-linked list.
struct Solution {}
impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut l1 = l1;
        let mut l2 = l2;
        let mut dummy = ListNode::new(0); // 哨兵节点
        let mut cur = &mut dummy;
        let mut carry = 0; // 进位
        while l1.is_some() || l2.is_some() || carry != 0 {
            let mut sum = carry;
            if let Some(node) = l1 {
                sum += node.val; // 节点值和进位加在一起
                l1 = node.next; // 下一个节点
            }
            if let Some(node) = l2 {
                sum += node.val; // 节点值和进位加在一起
                l2 = node.next; // 下一个节点
            }
            carry = sum / 10; // 新的进位
            cur.next = Some(Box::new(ListNode::new(sum % 10))); // 每个节点保存一个数位
            cur = cur.next.as_mut().unwrap(); // 下一个节点
        }
        dummy.next // 哨兵节点的下一个节点就是头节点
    }
}

