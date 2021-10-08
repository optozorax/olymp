// https://leetcode.com/problems/kth-smallest-element-in-a-bst/
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
    int kthSmallest(TreeNode* root, int k) {
        return find(root, k);
    }
    
    int find(TreeNode* current, int pos) {
        int left = count(current->left);
        int right = count(current->right);
        
        if (pos == left + 1) return current->val;
        
        if (pos > left + 1) {
            return find(current->right, pos - (left+1));
        } else {
            return find(current->left, pos);
        }
    }
    
    int count(TreeNode* root) {
        if (root == nullptr) return 0;
        return 1 + count(root->left) + count(root->right);
    }
};