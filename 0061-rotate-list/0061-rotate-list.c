/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     struct ListNode *next;
 * };
 */
struct ListNode* rotateRight(struct ListNode* head, int k){
    if (!head || !head->next || k == 0) return head;
    
    // find where the list should break
    struct ListNode* new_tail = head;
    struct ListNode* cur_node = head;
    int counter = 0;
    int num_of_nodes = 1;
    while (cur_node && cur_node->next) {
        cur_node = cur_node->next;
        if (counter == k) {
            new_tail = new_tail->next;
        } else {
            counter += 1;
        }
        num_of_nodes += 1;
    }
    
    // make sure we have correct new_end_node
    if (counter != k) {
        // invalid new_end_node because k was bigger than len of list
        int correct_k = k % num_of_nodes;
        if (correct_k == 0)
            return head;
        int distance_from_new_tail = (num_of_nodes - correct_k - 1);
        for (int i = 0; i < distance_from_new_tail; i += 1) {
            new_tail = new_tail->next;
        }
    }
    
    // set new end node and new head
    struct ListNode* new_head = new_tail->next;
    cur_node->next = head;
    new_tail->next = NULL;
    
    return new_head;
}