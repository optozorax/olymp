// https://leetcode.com/problems/path-sum-ii/
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
    vector<vector<int>> pathSum(TreeNode* root, int targetSum) {
        vector<vector<int>> result;
        vector<int> current;
        recur(root, result, current, targetSum, 0);
        return result;
    }
    
    void recur(
        TreeNode* root, 
        vector<vector<int>>& result, 
        vector<int>& current, 
        int target, 
        int sum
    ) {
        if (root == nullptr) return;
        
        current.push_back(root->val);
        sum += root->val;
        
        if (root->left == nullptr && root->right == nullptr) {
            if (sum == target) {
                result.push_back(current);
            }
            current.pop_back();
            return;
        }
        
        recur(root->left, result, current, target, sum);
        recur(root->right, result, current, target, sum);
        
        current.pop_back();
    }
};