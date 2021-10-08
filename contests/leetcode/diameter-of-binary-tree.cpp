// https://leetcode.com/problems/diameter-of-binary-tree/
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
    struct Result {
        int max_depth;
        int max_diameter;
    };

    int diameterOfBinaryTree(TreeNode* root) {
        return bfs(root).max_diameter;
    }

    Result bfs(TreeNode* root) {
        if (root->left == nullptr && root->right == nullptr) {
            return Result{0, 0};
        }
        
        Result res = {0, 0};
        
        int left_depth = 0;
        if (root->left != nullptr) {
            auto left = bfs(root->left);
            left_depth = left.max_depth + 1;
            res.max_depth = max(res.max_depth, left_depth);
            res.max_diameter = max(res.max_diameter, left_depth);
            res.max_diameter = max(res.max_diameter, left.max_diameter);
        }
        
        int right_depth = 0;
        if (root->right != nullptr) {
            auto right = bfs(root->right);
            right_depth = right.max_depth + 1;
            res.max_depth = max(res.max_depth, right_depth);
            res.max_diameter = max(res.max_diameter, right_depth);
            res.max_diameter = max(res.max_diameter, right.max_diameter);
        }
        
        res.max_diameter = max(res.max_diameter, left_depth + right_depth);

        return res;
    }
};