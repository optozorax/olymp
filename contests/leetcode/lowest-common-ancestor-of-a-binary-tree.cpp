// https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-tree/
/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     TreeNode *left;
 *     TreeNode *right;
 *     TreeNode(int x) : val(x), left(NULL), right(NULL) {}
 * };
 */
class Solution {
public:
    TreeNode* lowestCommonAncestor(TreeNode* root, TreeNode* p, TreeNode* q) {
        if (root == nullptr) return nullptr;
        
        auto left = lowestCommonAncestor(root->left, p, q);
        auto right = lowestCommonAncestor(root->right, p, q);
        
        if (left == nullptr && ((root == p && right == q) || (root == q && right == p))) 
            return root;
        if (right == nullptr && ((root == p && left == q) || (root == q && left == p)))
            return root;
        
        if ((left == p && right == q) || (left == q && right == p)) 
            return root;
        
        if (root == p) return p;
        if (root == q) return q;
        
        if (left == nullptr) return right;
        if (right == nullptr) return left;
        
        return nullptr;
    }
};