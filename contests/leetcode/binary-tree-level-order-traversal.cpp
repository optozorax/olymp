// https://leetcode.com/problems/binary-tree-level-order-traversal/
/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     TreeNode *left;
 *     TreeNode *right;
 *     TreeNode() : val(0), left(nullptr), right(nullptr) {}
 *     TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
 *     TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
 * };
 */
class Solution {
public:
    vector<vector<int>> levelOrder(TreeNode* root) {
        vector<vector<int>> result;
        recur(result, root, 0);
        return result;
    }
    
    void recur(vector<vector<int>>& result, TreeNode* root, int depth) {
        if (root == nullptr) return;
        
        if (result.size() < depth + 1) {
            result.push_back({});
        }
        
        result[depth].push_back(root->val);
        
        recur(result, root->left, depth+1);
        recur(result, root->right, depth+1);
    }
};