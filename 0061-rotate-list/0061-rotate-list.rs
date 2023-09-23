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
    pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if head.is_none() || head.as_ref().unwrap().next.is_none() {
            return head;
        }

        let node_len = count_len(&head);
        let mut nodes: Vec<Box<ListNode>> = Vec::with_capacity(node_len);
        for _ in 0..node_len {
            nodes.push(Box::new(ListNode::new(0)));
        }

        // This index points to the old head node in the vector
        let index = (k as usize) % node_len;
        move_nodes_into_vec(&mut nodes, head, index);

        for i in 0..(node_len - 1) {
            let index = node_len - i - 2;
            nodes[index].next = nodes.pop();
        }

        nodes.pop()
    }
}

fn count_len(head: &Option<Box<ListNode>>) -> usize {
    let mut cur_node = head;
    let mut count = 0;
    while let Some(ref node) = cur_node {
        cur_node = &node.next;
        count += 1;
    }
    count
}

fn move_nodes_into_vec(nodes: &mut Vec<Box<ListNode>>, node: Option<Box<ListNode>>, starting_index: usize) {
    let mut cur_node = node;
    let node_len = nodes.len();
    let mut index = starting_index;
    while let Some(mut node) = cur_node {
        let next_node = node.next.take();
        nodes[index] = node;
        cur_node = next_node;
        index = (index + 1) % node_len;
    }
}
