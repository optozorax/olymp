// https://leetcode.com/problems/odd-even-linked-list/
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
    ListNode* oddEvenList(ListNode* head) {
        if (head == nullptr) return head;
        if (head->next == nullptr) return head;
        
        auto i = head;
        auto j = head->next;
        
        auto head1 = i;
        auto head2 = j;
        
        while (j != nullptr) {
            i->next = j->next;
            if (j->next != nullptr) {
                j->next = j->next->next;
            }
            i = i->next;
            j = j->next;
        }

        i = head1;
        while (i->next != nullptr) 
            i = i->next;
        i->next = head2;
        
        return head1;
    }
};