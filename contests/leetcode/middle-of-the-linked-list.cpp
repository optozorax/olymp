// https://leetcode.com/problems/middle-of-the-linked-list/
/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode() : val(0), next(nullptr) {}
 *     ListNode(int x) : val(x), next(nullptr) {}
 *     ListNode(int x, ListNode *next) : val(x), next(next) {}
 * };
 */
class Solution {
public:
    ListNode* middleNode(ListNode* head) {
        auto walker = head;
        auto runner = head;
        while (true) {
            if (runner->next == nullptr) { // list size is odd
                return walker;
            }
            if (runner->next->next == nullptr) { // list size is even
                return walker->next;
            }
            runner = runner->next->next;
            
            walker = walker->next;
        }
    }
};