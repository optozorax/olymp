// https://leetcode.com/problems/palindrome-linked-list/
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
    bool isPalindrome(ListNode* head) {
        auto n = sizeList(head);
        auto middle = middleNode(head, n);
        auto head2 = invertList(middle);
        auto result = isEqual(head, head2, n / 2);
        return result;
    }
    
    ListNode* middleNode(ListNode* head, int n) {
        auto head2 = head;
        int count = 0;
        for (auto i = head; i != nullptr; i = i->next) {
            if (count == n / 2 + n % 2) {
                head2 = i;
                break;
            }
            count++;
        }
        return head2;
    }
    
    int sizeList(ListNode* head) {
        int n = 0;
        for (auto i = head; i != nullptr; i = i->next) {
            n++;
        }
        return n;
    }
    
    ListNode* invertList(ListNode* head) {
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
    
    bool isEqual(ListNode* head1, ListNode* head2, int count) {
        while (true) {
            if (count == 0) return true;
            if (head1 == nullptr && head2 == nullptr) return true;
            if (head1 == nullptr || head2 == nullptr) return true;
            if (head1->val != head2->val) return false;
            head1 = head1->next;
            head2 = head2->next;
            count--;
        }
    }
};