// https://leetcode.com/problems/reverse-nodes-in-k-group/
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
    ListNode* reverseKGroup(ListNode* head, int k) {
        int n = listSize(head);
        auto before_head = new ListNode(0, head);
        auto previous = before_head;
        for (int i = 0; i < n / k; ++i) {
            auto current = offset(previous, k);
            auto next = current->next;
            current->next = nullptr;
            
            previous->next = reverseList(previous->next);
            current = offset(previous, k);
            current->next = next;
            
            previous = current;
        }
        return before_head->next;
    }
    
    int listSize(ListNode* head) {
        int n = 0;
        for (auto i = head; i != nullptr; i = i->next) {
            n++;
        }
        return n;
    }
    
    ListNode* offset(ListNode* a, int count) {
        if (a == nullptr) return nullptr;
        for (int i = 0; i < count; ++i) {
            if (a->next != nullptr) 
                a = a->next;
        }
        return a;
    }
    
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