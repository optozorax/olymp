// https://leetcode.com/problems/path-sum-iii/
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
    int pathSum(TreeNode* root, int targetSum) {
        int count = 0;
        vector<int> prefix;
        prefix.push_back(0);
        
        recur(root, prefix, targetSum, count);
        
        return count;
    }
    
    void recur(TreeNode* root, vector<int>& prefix, int target, int& count) {
        if (root == nullptr) return;
        
        prefix.push_back(prefix.back() + root->val);
        
        for (int i = 0; i < prefix.size() - 1; ++i) {
            if (prefix.back() - prefix[i] == target) {
                count++;
            }
        }
        
        recur(root->left, prefix, target, count);
        recur(root->right, prefix, target, count);
        
        prefix.pop_back();
    }
};