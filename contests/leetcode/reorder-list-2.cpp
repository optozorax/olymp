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
        // находим количество
        int n = 0;
        for (auto i = head; i != nullptr; i = i->next) {
            n++;
        }

        // находим середину
        auto head2 = head;
        int count = 0;
        for (auto i = head; i != nullptr; i = i->next) {
            if (count == n / 2 + n % 2) {
                head2 = i;
                break;
            }
            count++;
        }

        // инвертируем середину
        {
            auto i = head2;
            ListNode* previous = nullptr;
            while (i != nullptr) {
                head2 = i;
                auto next = i->next;
                i->next = previous;
                previous = i;
                i = next;
            }
        }

        // мерджим
        {
            auto i = head;
            auto j = head2;
            while (j != nullptr) {
                auto nexti = i->next;
                auto nextj = j->next;
                i->next = j;
                j->next = nexti;
                i = nexti;
                j = nextj;
            }
            if (i != nullptr) i->next = nullptr;
        }
    }
};

/* 

Идеальное решение:
	* Инвертируем вторую половинку
	* Проходимся по обеим половинкам и чередуем их

 */