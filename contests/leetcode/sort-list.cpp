// https://leetcode.com/problems/sort-list/
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
    ListNode* sortList(ListNode* head) {
        if (head == nullptr) return nullptr;
        int n = listSize(head);
        
        auto before_head = new ListNode();
        before_head->next = head;
        
        int count = next_pow2(n);
        
        int size = 1;
        while (size < count) {
            auto previous = before_head;
            auto current = previous->next;
            int j = n;
            while (j > size) {
                auto a = current;
                auto b = offset(current, size);
                auto next = offset_null(b, size);
                
                offset(a, size-1)->next = nullptr; // erase pointer to b
                offset(b, size-1)->next = nullptr; // erase pointer to next
                
                previous->next = mergeTwoLists(a, b);
                
                auto end = offset(previous->next, size * 2 - 1);
                end->next = next;
                
                previous = end;
                current = next;
                
                j -= size * 2;
            }
            size *= 2;
        }
        return before_head->next;
    }
    
    void printList(ListNode* head) {
        for (auto i = head; i != nullptr; i = i->next) {
            cout << i->val << " ";
        }
        cout << endl;
    }
    
    ListNode* offset(ListNode* a, int count) {
        if (a == nullptr) return nullptr;
        for (int i = 0; i < count; ++i) {
            if (a->next != nullptr) 
                a = a->next;
        }
        return a;
    }
    
    ListNode* offset_null(ListNode* a, int count) {
        if (a == nullptr) return nullptr;
        for (int i = 0; i < count; ++i) {
            if (a != nullptr) 
                a = a->next;
        }
        return a;
    }
    
    uint32_t next_pow2(uint32_t x) {
        return x == 1 ? 1 : 1<<(32-__builtin_clz(x-1));
    }
    
    int listSize(ListNode* head) {
        int n = 0;
        for (auto i = head; i != nullptr; i = i->next) {
            n++;
        }
        return n;
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