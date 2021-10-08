// https://leetcode.com/problems/linked-list-cycle-ii/
/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode(int x) : val(x), next(NULL) {}
 * };
 */
class Solution {
public:
    ListNode *detectCycle(ListNode *head) {
        if (head == nullptr) return nullptr;
        
        auto walker = head;
        auto runner = head;
        while (true) {
            if (walker->next == nullptr) return nullptr;
            walker = walker->next;

            if (runner->next == nullptr) return nullptr;
            if (runner->next->next == nullptr) return nullptr;
            runner = runner->next->next;

            if (walker == runner) break;
        }
        
        walker = head;
        while (true) {
            if (walker == runner) return walker;
            
            walker = walker->next;
            runner = runner->next;
        }
    }
};