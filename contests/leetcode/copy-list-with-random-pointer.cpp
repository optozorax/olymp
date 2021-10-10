// https://leetcode.com/problems/copy-list-with-random-pointer/
/*
// Definition for a Node.
class Node {
public:
    int val;
    Node* next;
    Node* random;
    
    Node(int _val) {
        val = _val;
        next = NULL;
        random = NULL;
    }
};
*/

class Solution {
public:
    Node* copyRandomList(Node* head) {
        if (head == nullptr) return nullptr;
        
        auto i = head;
        while (i != nullptr) {
            auto next = i->next;
            i->next = new Node(i->val);
            i->next->next = next;
            i = i->next->next;
        }
        
        i = head;
        while (i != nullptr) {
            if (i->random != nullptr) {
                i->next->random = i->random->next;
            } else {
                i->next->random = nullptr;
            }
            i = i->next->next;
        }
        
        i = head;
        auto answer = i->next;
        while (i != nullptr) {
            auto next = i->next;
            i->next = i->next->next;
            if (next->next != nullptr) {
                 next->next = next->next->next;
            } else {
                next->next = nullptr;
            }
            
            i = i->next;
        }
        
        return answer;
    }
};