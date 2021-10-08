// https://leetcode.com/problems/remove-duplicates-from-sorted-list/
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
    ListNode* deleteDuplicates(ListNode* head) {
        auto previous_val = -1000;
        while (head != nullptr && head->val == previous_val) {
            previous_val = head->val;
            head = head->next;
        }
        
        auto i = head;
        auto previous = head;
        while (i != nullptr) {
            if (i->val == previous_val) {
                previous->next = i->next;
            } else {
                previous = i;
            }
            previous_val = i->val;
            i = i->next;
        }

        return head;
    }
};