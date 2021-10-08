// https://leetcode.com/problems/reorder-list/
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
    void reorderList(ListNode* head) {
        vector<ListNode*> nodes;
        auto i = head;
        while (i != nullptr) {
            nodes.push_back(i);
            i = i->next;
        }
        
        for (int i = 0; i < nodes.size() / 2; ++i) {
            nodes[i]->next = nodes[(nodes.size() - 1) - i];
            nodes[(nodes.size() - 1) - i]->next = nodes[i+1];
        }
        nodes[nodes.size() / 2]->next = nullptr;
    }
};

/* 

Идеальное решение:
	* Инвертируем вторую половинку
	* Проходимся по обеим половинкам и чередуем их

 */