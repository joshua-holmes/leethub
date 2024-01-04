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
    pub fn odd_even_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy_even = ListNode::new(-1);
        let mut dummy_odd = ListNode::new(-1);
        let mut even = &mut dummy_even;
        let mut odd = &mut dummy_odd;

        let mut is_even = true;
        let mut node = &mut head;

        while let Some(n) = node.take() {
            if is_even {
                even.next = Some(n);
                even = even.next.as_mut().unwrap().as_mut();
                node = &mut even.next; 
            } else {
                odd.next = Some(n);
                odd = odd.next.as_mut().unwrap().as_mut();
                node = &mut odd.next; 
            }
            is_even = !is_even;
        }

        even.next = dummy_odd.next;
        dummy_even.next
    }
}