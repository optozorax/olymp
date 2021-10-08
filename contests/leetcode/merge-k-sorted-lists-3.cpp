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
        priority_queue<
            pair<int, ListNode*>, 
            vector<pair<int, ListNode*>>, 
            greater<pair<int, ListNode*>>
        > heap;
        for (auto& i : lists) {
            if (i != nullptr)
                heap.push(make_pair(i->val, i));
        }
        
        ListNode* before_head = new ListNode(0, nullptr);
        
        auto previous = before_head;
        
        while (heap.size() != 0) {
            auto top = heap.top().second;
            heap.pop();

            if (top->next != nullptr) {
                heap.push(make_pair(top->next->val, top->next));
            }
            
            previous->next = top;
            previous = previous->next;
        }
        
        return before_head->next;
    }
};