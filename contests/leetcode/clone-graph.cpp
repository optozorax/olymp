// https://leetcode.com/problems/clone-graph/
/*
// Definition for a Node.
class Node {
public:
    int val;
    vector<Node*> neighbors;
    Node() {
        val = 0;
        neighbors = vector<Node*>();
    }
    Node(int _val) {
        val = _val;
        neighbors = vector<Node*>();
    }
    Node(int _val, vector<Node*> _neighbors) {
        val = _val;
        neighbors = _neighbors;
    }
};
*/

class Solution {
public:
    Node* cloneGraph(Node* node) {
        if (node == nullptr) return nullptr;
        unordered_map<Node*, Node*> cloned;
        return recur(node, cloned);
    }
    
    Node* recur(Node* node, unordered_map<Node*, Node*>& cloned) {
        if (cloned[node] != nullptr) return cloned[node];
        
        auto new_node = new Node(node->val);
        
        cloned[node] = new_node;
        
        for (const auto& i : node->neighbors) {
            new_node->neighbors.push_back(recur(i, cloned));
        }
        
        return new_node;
    }
};