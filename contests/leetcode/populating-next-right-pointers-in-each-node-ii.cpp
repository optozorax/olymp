// https://leetcode.com/problems/populating-next-right-pointers-in-each-node-ii/
/*
// Definition for a Node.
class Node {
public:
    int val;
    Node* left;
    Node* right;
    Node* next;

    Node() : val(0), left(NULL), right(NULL), next(NULL) {}

    Node(int _val) : val(_val), left(NULL), right(NULL), next(NULL) {}

    Node(int _val, Node* _left, Node* _right, Node* _next)
        : val(_val), left(_left), right(_right), next(_next) {}
};
*/

class Solution {
public:
    Node* connect(Node* root) {
        vector<Node*> leftmost;
        recur(root, leftmost, 0);
        return root;
    }
    
    void recur(Node* root, vector<Node*>& leftmost, int depth) {
        if (root == nullptr) return;
        
        if (leftmost.size() < depth + 1) leftmost.push_back(nullptr);
        
        root->next = leftmost[depth];
        leftmost[depth] = root;
        
        recur(root->right, leftmost, depth+1);
        recur(root->left, leftmost, depth+1);
    }
};