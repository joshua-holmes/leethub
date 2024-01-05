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
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() || head.as_ref().unwrap().next.is_none() {
            return head;
        }
        let mut prev_node = head;
        let mut node = prev_node.as_mut().unwrap().as_mut().next.take();
        let mut next_node = node.as_mut().unwrap().as_mut().next.take();
        while let Some(mut n) = node.take() {
            n.next = prev_node;
            prev_node = Some(n);
            node = next_node;
            next_node = if let Some(nn) = node.as_mut() {
                nn.as_mut().next.take()
            } else {
                None
            };
        }

        prev_node
    }
}