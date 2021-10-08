// https://leetcode.com/problems/reverse-linked-list/
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
    ListNode* reverseList(ListNode* head) {
        auto i = head;
        ListNode* previous = nullptr;
        while (i != nullptr) {
            head = i;
            auto next = i->next;
            i->next = previous;
            previous = i;
            i = next;
        }
        return head;
    }
};