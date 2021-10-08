// https://leetcode.com/problems/binary-tree-maximum-path-sum/
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
    int maxPathSum(TreeNode* root) {
        return recur(root).first;
    }
    
    pair<int, int> recur(TreeNode* root) { // pair<own, to_top>
        if (root == nullptr) return make_pair(-1000000000, -1000000000);
        
        auto l = recur(root->left);
        auto r = recur(root->right);

        int own = root->val;
        own = max(own, l.first);
        own = max(own, r.first);
        own = max(own, root->val + r.second);
        own = max(own, root->val + l.second);
        own = max(own, root->val + r.second + l.second);
        
        int to_top = root->val;
        to_top = max(to_top, root->val + r.second);
        to_top = max(to_top, root->val + l.second);
        
        return make_pair(own, to_top);
    }
};