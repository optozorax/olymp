// https://leetcode.com/problems/path-sum/
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
    bool hasPathSum(TreeNode* root, int targetSum) {
        if (root == nullptr) return false;
        return bfs(root, 0, targetSum);
    }
    
    bool bfs(TreeNode* root, int sum, int target) {
        if (root == nullptr) return false;
        if (root->left == nullptr && root->right == nullptr) return root->val + sum == target;
        return 
            bfs(root->left, sum + root->val, target) ||
            bfs(root->right, sum + root->val, target);
    }
};