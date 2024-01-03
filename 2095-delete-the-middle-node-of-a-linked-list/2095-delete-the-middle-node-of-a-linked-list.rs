// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn delete_middle(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut node = &head;
        let mut count = 0;
        while let Some(inside_node) = node {
            count += 1;
            node = &inside_node.next;
        }

        if count == 1 {
            return None;
        }
        let mut node = head.as_mut().unwrap().as_mut();
        let middle = (count - 2) / 2;
        for _ in 0..middle {
            node = node.next.as_mut().unwrap().as_mut();
        }
        node.next = node.next.as_mut().unwrap().as_mut().next.take();

        head

    }
}