use std::collections::HashSet;
use std::iter::once;
use crate::common_struct::{ListNode, Solution};
// https://leetcode.cn/problems/remove-duplicate-node-lcci/?envType=study-plan-v2&envId=cracking-the-coding-interview
// 编写代码，移除未排序链表中的重复节点。保留最开始出现的节点。
//
// 示例1：
//
//  输入：[1, 2, 3, 3, 2, 1]
//  输出：[1, 2, 3]
// 示例2：
//
//  输入：[1, 1, 1, 1, 2]
//  输出：[1, 2]
// 提示：
//
// 链表长度在[0, 20000]范围内。
// 链表元素在[0, 20000]范围内。
// 进阶：
//
// 如果不得使用临时缓冲区，该怎么解决？
// 用快慢指针

impl Solution {
    /// 使用 vec 保存，也可使用 set
    pub fn remove_duplicate_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        head.map(|mut head| {
            let mut cur = head.as_mut();
            let mut bucket = vec![0; 20001];
            bucket[cur.val as usize] = 1;
            while let Some(next) = cur.next.as_mut() {
                if bucket[next.val as usize] != 0 {
                    cur.next = next.next.take();
                } else {
                    bucket[next.val as usize] = 1;
                    cur = cur.next.as_mut().unwrap();
                }
            }
            head
        })
    }
}

impl Solution {
    pub fn remove_duplicate_nodes_3(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None
        }
        let mut head= head;
        let mut prev = head.as_mut().unwrap();
        let mut d = once(prev.val).collect::<HashSet<i32>>();
        while prev.next.is_some() {
            let next = prev.next.as_ref().unwrap();
            if !d.contains(&next.val){
                d.insert(next.val);
                prev = prev.next.as_mut().unwrap();
            } else {
                prev.next = prev.next.take().unwrap().next;
            }
        }
        head
    }
}

impl Solution {
    /// 土方法，使用 set vec 保存链表不重复的数据，然后重新插入
    pub fn remove_duplicate_nodes_2(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut cur = head;
        let mut set = HashSet::new();
        let mut v = Vec::new();
        while let Some(node) = cur {
            if !set.contains(&(node.val)) {
                v.push(node.val);
                set.insert(node.val);
            }
            cur = node.next;
        }
        v.reverse();
        let mut prev = None;
        for i in 0..v.len() {
            let last = v[i];
            let mut node = ListNode::new(last);
            node.next = prev;
            prev = Some(Box::new(node));
            set.insert(last);
        }
        prev
    }
}

#[cfg(test)]
mod test {
    use crate::common_struct::{ListNode, Solution};

    #[test]
    fn test_remove_duplicate_nodes() {
        let node3 = Some(Box::new(ListNode { val: 3, next: None }));
        let node2 = Some(Box::new(ListNode { val: 2, next: node3 }));
        let node1 = Some(Box::new(ListNode { val: 2, next: node2 }));
        let head = Some(Box::new(ListNode { val: 1, next: node1 }));
        let new_head = Solution::remove_duplicate_nodes(head);
        println!("{:?}", new_head);
    }

    #[test]
    fn test_remove_duplicate_nodes_2() {
        let node3 = Some(Box::new(ListNode { val: 3, next: None }));
        let node2 = Some(Box::new(ListNode { val: 2, next: node3 }));
        let node1 = Some(Box::new(ListNode { val: 2, next: node2 }));
        let head = Some(Box::new(ListNode { val: 1, next: node1 }));
        let new_head = Solution::remove_duplicate_nodes_2(head);
        println!("{:?}", new_head);
    }

    #[test]
    fn test_remove_duplicate_nodes_3() {
        let node3 = Some(Box::new(ListNode { val: 3, next: None }));
        let node2 = Some(Box::new(ListNode { val: 2, next: node3 }));
        let node1 = Some(Box::new(ListNode { val: 2, next: node2 }));
        let head = Some(Box::new(ListNode { val: 1, next: node1 }));
        let new_head = Solution::remove_duplicate_nodes_3(head);
        println!("{:?}", new_head);
    }

}