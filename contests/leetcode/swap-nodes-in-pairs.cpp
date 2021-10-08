// https://leetcode.com/problems/swap-nodes-in-pairs/
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
    ListNode* swapPairs(ListNode* head) {
        if (head == nullptr) return nullptr;
        if (head->next == nullptr) return head;
        
        auto before_head = new ListNode(0, head);

        auto j = head;
        auto previous = before_head;
        while (j != nullptr && j->next != nullptr) {
            auto s1 = j;
            auto s2 = j->next; // 2
            auto s3 = j->next->next; // 3
            
            s2->next = s1; // 2->1
            s1->next = s3; // 1->3
            previous->next = s2; // 0->2
            
            previous = j;
            j = s3;
        }
        
        return before_head->next;
    }
};