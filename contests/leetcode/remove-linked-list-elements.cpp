// https://leetcode.com/problems/remove-linked-list-elements/
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
    ListNode* removeElements(ListNode* head, int val) {
        while (head != nullptr && head->val == val) 
            head = head->next;
        
        auto i = head;
        auto previous = head;
        while (i != nullptr) {
            if (i->val == val) {
                previous->next = i->next;
            } else {
                previous = i;
            }
            i = i->next;
        }

        return head;
    }
};