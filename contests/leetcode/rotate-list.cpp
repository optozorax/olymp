// https://leetcode.com/problems/rotate-list/
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
    ListNode* rotateRight(ListNode* head, int k) {
        if (head == nullptr) return nullptr;
        
        int n = listSize(head);
        k = k % n;
        
        if (k == 0) return head;
        
        auto before_rot = offset(head, n-k-1);
        auto new_start = before_rot->next;
        before_rot->next = nullptr;
        offset(new_start, k-1)->next = head;
        
        return new_start;
    }
    
    ListNode* offset(ListNode* a, int count) {
        if (a == nullptr) return nullptr;
        for (int i = 0; i < count; ++i) {
            if (a->next != nullptr) 
                a = a->next;
        }
        return a;
    }
    
    int listSize(ListNode* head) {
        int n = 0;
        for (auto i = head; i != nullptr; i = i->next) {
            n++;
        }
        return n;
    }
};