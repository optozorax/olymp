// https://leetcode.com/problems/binary-tree-right-side-view/
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
    vector<int> rightSideView(TreeNode* root) {
        vector<int> rights;
        recur(root, rights, 0);
        return rights;
    }
    
    void recur(TreeNode* root, vector<int>& rights, int depth) {
        if (root == nullptr) return;
        
        if (rights.size() < depth + 1) {
            rights.push_back(root->val);
        }
        
        recur(root->right, rights, depth+1);
        recur(root->left, rights, depth+1);
    }
};