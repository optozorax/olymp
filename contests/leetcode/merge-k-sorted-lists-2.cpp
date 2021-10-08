// https://leetcode.com/problems/merge-k-sorted-lists/
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
    ListNode* mergeKLists(vector<ListNode*>& lists) {
        if (lists.size() == 0) return nullptr;
        if (lists.size() == 1) return lists[0];
        
        vector<ListNode*> lists2;
        while (lists.size() > 1) {
            lists2.clear();
            for (int i = 1; i < lists.size(); i += 2) {
                lists2.push_back(mergeTwoLists(lists[i-1], lists[i]));
            }
            if (lists.size() % 2 == 1) lists2.push_back(lists.back());
            swap(lists, lists2);
        }
        
        return lists[0];
    }
    
    ListNode* mergeTwoLists(ListNode* l1, ListNode* l2) {
        if (l1 == nullptr) return l2;
        if (l2 == nullptr) return l1;
        
        auto a = l1;
        auto b = l2;
        if (a->val > b->val) swap(a, b);
        
        auto result = a;
        
        while (a != nullptr && b != nullptr) {
            // we guarantee that a->val <= b->val
            if (a->next == nullptr || a->next->val > b->val) {
                auto next = a->next;
                a->next = b;
                b = next;
            } else {
                a = a->next;
            }
        }
        
        return result;
    }
};
