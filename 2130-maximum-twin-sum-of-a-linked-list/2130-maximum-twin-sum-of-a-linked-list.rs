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
    pub fn pair_sum(head: Option<Box<ListNode>>) -> i32 {
        let mut node = &head;
        let mut length: i32 = 0;
        while let Some(n) = node.as_ref() {
            length += 1;
            node = &n.next;
        }

        let mut values = Vec::with_capacity(length as usize / 2);
        let mut i: i32 = 0;
        let half = length / 2;
        let mut node = &head;
        while let Some(n) = node.as_ref() {
            let v_index = if i >= half {
                half - (i % half) - 1
            } else {
                i
            };
            
            if let Some(v) = values.get_mut(v_index as usize) {
                *v += n.val;
            } else {
                values.push(n.val);
            }
            
            node = &n.next;
            i += 1;
        }
                
        values.into_iter().max().unwrap_or(0)
    }
}
