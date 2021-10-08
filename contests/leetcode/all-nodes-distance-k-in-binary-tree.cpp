// https://leetcode.com/problems/all-nodes-distance-k-in-binary-tree/
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
    vector<int> distanceK(TreeNode* root, TreeNode* target, int k) {
        if (k == 0) return {target->val};
        vector<int> result;
        recur(root, target, result, k, -1);
        return result;
    }
    
    int recur(TreeNode* root, TreeNode* target, vector<int>& result, int k, int depth) {
        if (root == nullptr) 
            return -1; // ?
        if (depth == k) {
            result.push_back(root->val);
            return -1;
        }
        
        if (root == target) {
            recur(root->left, target, result, k, 1);
            recur(root->right, target, result, k, 1);
            return 1;
        } else {
            if (depth != -1) {
                recur(root->left, target, result, k, depth + 1);    
                recur(root->right, target, result, k, depth + 1);    
                return -1;
            } else {
                int left = recur(root->left, target, result, k, -1);
                int right = recur(root->right, target, result, k, -1);
                if (left != -1) {
                    if (left == k) result.push_back(root->val);
                    recur(root->right, target, result, k, left + 1);
                    return left + 1;
                }
                if (right != -1) {
                    if (right == k) result.push_back(root->val);
                    recur(root->left, target, result, k, right + 1);
                    return right + 1;
                }
                return -1;
            }
        }
    }
};