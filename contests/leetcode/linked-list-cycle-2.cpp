// https://leetcode.com/problems/linked-list-cycle/
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
    bool hasCycle(ListNode *head) {
        if (head == nullptr) return false;
        
        auto walker = head;
        auto runner = head;
        while (true) {
            if (walker->next == nullptr) return false;
            walker = walker->next;

            if (runner->next == nullptr) return false;
            if (runner->next->next == nullptr) return false;
            runner = runner->next->next;

            if (walker == runner) return true;
        }
    }
};