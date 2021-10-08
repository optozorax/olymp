// https://leetcode.com/problems/remove-nth-node-from-end-of-list/
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
    ListNode* removeNthFromEnd(ListNode* head, int back) {
        if (head == nullptr) return nullptr;
        
        int n = 0;
        for (auto i = head; i != nullptr; i = i->next) {
            n++;
        }
        
        if (n-back == 0) return head->next;
        
        auto j = head;
        for (int i = 0; i < n - back - 1; ++i) {
            j = j->next;  
        }
        j->next = j->next->next;
        
        return head;
    }
};