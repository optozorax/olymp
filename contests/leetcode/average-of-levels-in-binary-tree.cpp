// https://leetcode.com/problems/average-of-levels-in-binary-tree/
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
    vector<double> averageOfLevels(TreeNode* root) {
        vector<double> sum;
        vector<double> count;
        bfs(root, sum, count, 0);
        
        vector<double> result(sum.size(), 0);
        for (int i = 0; i < sum.size(); ++i) {
            result[i] = double(sum[i]) / double(count[i]);
        }
        return result;
    }
    
    void bfs(TreeNode* root, vector<double>& sum, vector<double>& count, int level) {
        if (root == nullptr) return;
        
        if (level+1 > sum.size()) sum.push_back(0);
        if (level+1 > count.size()) count.push_back(0);
        
        sum[level] += root->val;
        count[level] += 1;
        
        bfs(root->left, sum, count, level+1);
        bfs(root->right, sum, count, level+1);
    }
};