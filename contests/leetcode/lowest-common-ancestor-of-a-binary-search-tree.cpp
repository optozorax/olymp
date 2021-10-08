// https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-search-tree/
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
        int p1 = p->val;
        int q1 = q->val;
        if (p1 > q1) swap(p1, q1);
        return lca(root, p1, q1);
    }
    
    TreeNode* lca(TreeNode* root, int p, int q) {
        if (root->val > p && root->val > q) return lca(root->left, p, q);
        if (root->val < p && root->val < q) return lca(root->right, p, q);
        
        return root;
    }
};