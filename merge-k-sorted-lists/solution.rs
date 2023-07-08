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
struct Min {
    val: Option<i32>,
    index: usize
}
impl Min {
    fn new(val: Option<i32>, index: usize) -> Self {
        Self { val, index }
    }
}

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut result = Box::new(ListNode::new(0));
        let mut cur_node = &mut result;
        let mut l_copy = lists.to_vec();

        while l_copy.iter().any(|l| l.is_some()) {

            // Find node with min value
            let mut min = Min::new(None, 0);
            for (i, list) in l_copy.iter().enumerate() {
                if let Some(l) = list {
                    min = match min.val {
                        Some(m) => {
                            if l.val < m {
                                Min::new(Some(l.val), i)
                            } else {
                                min
                            }
                        }
                        None => Min::new(Some(l.val), i)
                    };
                }
            }

            // If found, add new node to result
            if let Some(v) = min.val {
                cur_node.next = Some(Box::new(ListNode::new(v)));
                cur_node = cur_node.next.as_mut().unwrap();
                let mut l_node = &l_copy[min.index];
                l_node = &(l_node.as_ref().unwrap().next);
            } else {
                break;
            }
        }

        result.next
    }
}
