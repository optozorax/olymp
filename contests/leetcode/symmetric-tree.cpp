// https://leetcode.com/problems/symmetric-tree/
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
    bool isSymmetric(TreeNode* root) {
        if (root == nullptr) return true;
        return is_symmetric(root->left, root->right);
    }
    
    bool is_symmetric(TreeNode* left, TreeNode* right) {
        if (left == nullptr && right == nullptr) return true;
        if (left == nullptr || right == nullptr) return false;
        return 
            left->val == right->val &&
            is_symmetric(left->right, right->left) &&
            is_symmetric(left->left, right->right);
    }
};