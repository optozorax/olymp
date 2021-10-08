// https://leetcode.com/problems/maximum-width-of-binary-tree/
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
    int widthOfBinaryTree(TreeNode* root) {
        vector<int64_t> lefts;
        int64_t best = 1;
        recur(root, lefts, best, 1, 0);
        return best;
    }
    
    void recur(TreeNode* root, vector<int64_t>& lefts, int64_t& best, int64_t pos, int depth) {
        if (root == nullptr) return;
        if (lefts.size() < depth + 1) lefts.push_back(pos);
        best = max(best, pos - lefts[depth] + 1);
        recur(root->left, lefts, best, (pos-lefts[depth]) * 2 + 1, depth+1);
        recur(root->right, lefts, best, (pos-lefts[depth]) * 2 + 2, depth+1);    
    }
};