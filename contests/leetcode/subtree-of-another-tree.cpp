https://leetcode.com/problems/subtree-of-another-tree/
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
    bool isSubtree(TreeNode* root, TreeNode* subRoot) {
        return bfs(root, subRoot);
    }
    
    bool bfs(TreeNode* root, TreeNode* sub) {
        if (is_equal(root, sub)) return true;
        if (root != nullptr) {
            if (bfs(root->left, sub)) return true;
            if (bfs(root->right, sub)) return true;
        }
        return false;
    }
    
    bool is_equal(TreeNode* a, TreeNode* b) {
        if (a == nullptr && b == nullptr) return true;
        if (a == nullptr || b == nullptr) return false;
        
        return 
            a->val == b->val && 
            is_equal(a->left, b->left) && 
            is_equal(a->right, b->right);
    }
};