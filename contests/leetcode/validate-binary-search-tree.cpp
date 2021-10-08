// https://leetcode.com/problems/validate-binary-search-tree/
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
    bool isValidBST(TreeNode* root) {
        return recur(root, nullopt, nullopt);
    }
    
    bool recur(TreeNode* root, optional<int> min, optional<int> max) {
        if (root == nullptr) return true;
        if (min && root->val <= *min) return false; 
        if (max && root->val >= *max) return false;
        return 
            recur(root->left, min, {root->val}) &&
            recur(root->right, {root->val}, max);
    
    }
};