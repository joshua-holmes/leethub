/**
 * Definition for singly-linked list.
 * function ListNode(val, next) {
 *     this.val = (val===undefined ? 0 : val)
 *     this.next = (next===undefined ? null : next)
 * }
 */
/**
 * @param {ListNode} head
 * @return {ListNode}
 */
var deleteDuplicates = function(head) {
    let node = head;
    while (node) {
        while (node.val === node.next?.val) {
            node.next = node.next.next
        }
        node = node.next;
    }
    
    return head;
};