// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}


impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }
}

pub struct Solution;


impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy_head = Some(Box::new(ListNode::new(0)));
        let mut curr = &mut dummy_head;
        let mut carry = 0;
        let mut p = l1;
        let mut q = l2;
        while p.is_some() || q.is_some() || carry > 0 {
            let mut sum = carry;
            if let Some(node) = p {
                sum += node.val;
                p = node.next;
            }
            if let Some(node) = q {
                sum += node.val;
                q = node.next;
            }
            carry = sum / 10;
            if let Some(node) = curr {
                node.next = Some(Box::new(ListNode::new(sum % 10)));
                curr = &mut node.next;
            }
        }
        dummy_head.unwrap().next
    }
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_add_two_numbers() {
        // 创建第一个链表 2 -> 4 -> 3 (表示数字 342)
        let l1 = Some(Box::new(ListNode { val: 2, next: Some(Box::new(ListNode { val: 4, next: Some(Box::new(ListNode { val: 3, next: None })) })) }));
        // 创建第二个链表 5 -> 6 -> 4 (表示数字 465)
        let l2 = Some(Box::new(ListNode { val: 5, next: Some(Box::new(ListNode { val: 6, next: Some(Box::new(ListNode { val: 4, next: None })) })) }));
        // 期望的结果 7 -> 0 -> 8 (表示数字 807)
        let expected = Some(Box::new(ListNode { val: 7, next: Some(Box::new(ListNode { val: 0, next: Some(Box::new(ListNode { val: 8, next: None })) })) }));


        let result = Solution::add_two_numbers(l1, l2);
        assert_eq!(result, expected);
    }


    #[test]
    fn test_add_two_numbers_with_zero() {
            // 创建第一个链表 0
            let l1 = Some(Box::new(ListNode::new(0)));
            // 创建第二个链表 0
            let l2 = Some(Box::new(ListNode::new(0)));
            // 期望的结果 0
            let expected = Some(Box::new(ListNode::new(0)));


            let result = Solution::add_two_numbers(l1, l2);
            assert_eq!(result, expected);
        }
    }