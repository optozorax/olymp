// https://leetcode.com/problems/add-two-numbers/
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
    ListNode* addTwoNumbers(ListNode* l1, ListNode* l2) {
        if (l1 == nullptr) return l2;
        if (l2 == nullptr) return l1;
        
        auto result = l1;
        
        while (l1 != nullptr) {
            if (l2 != nullptr) 
                l1->val += l2->val;
            
            if ((l2 != nullptr && l2->next != nullptr) || l1->val > 9) {
                if (l1->next == nullptr) 
                    l1->next = new ListNode();
            }
            
            if (l1->val > 9) {
                l1->next->val += l1->val / 10;
                l1->val = l1->val % 10;
            }
            
            l1 = l1->next;
            if (l2 != nullptr) l2 = l2->next;
        }
        
        return result;
    }
};