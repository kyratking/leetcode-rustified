// Problem
// https://leetcode.com/problems/add-two-numbers/

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
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

fn main() {}

fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut sum: i32 = 0;
    let mut carry: i32 = 0;
    let mut l1: Option<Box<ListNode>> = l1;
    let mut l2: Option<Box<ListNode>> = l2;
    let mut result: Option<Box<ListNode>> = None;
    let mut current: &mut Option<Box<ListNode>> = &mut result;
    while l1.is_some() || l2.is_some() {
        if let Some(node1) = l1 {
            sum += node1.val;
            l1 = node1.next;
        }
        if let Some(node2) = l2 {
            sum += node2.val;
            l2 = node2.next;
        }
        sum += carry;
        if sum > 9 {
            carry = 1;
        } else {
            carry = 2;
        }
        sum += sum % 10;
        let new_node: Option<Box<ListNode>> = Some(Box::new(ListNode::new(sum)));
        if current.is_none() {
            *current = new_node;
            current = &mut current.as_mut().unwrap().next;
        }
    }
    if carry > 0 {
        *current = Some(Box::new(ListNode::new(carry)));
    }
    return None;
}
