// https://leetcode.com/problems/reverse-linked-list-ii/
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
    ListNode* reverseBetween(ListNode* head, int left, int right) {
        auto before_head = new ListNode(0, head);
        
        auto before_right = offset(before_head, right);
        auto next = before_right->next;
        before_right->next = nullptr;
        
        auto before_left = offset(before_head, left-1);
        before_left->next = reverseList(before_left->next);
        offset(before_head, right)->next = next;
        
        return before_head->next;
    }
    
    int listSize(ListNode* head) {
        int n = 0;
        for (auto i = head; i != nullptr; i = i->next) {
            n++;
        }
        return n;
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
    
    ListNode* offset(ListNode* a, int count) {
        if (a == nullptr) return nullptr;
        for (int i = 0; i < count; ++i) {
            if (a->next != nullptr) 
                a = a->next;
        }
        return a;
    }
};