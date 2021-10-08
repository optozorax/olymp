// https://leetcode.com/problems/maximum-depth-of-binary-tree/
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
    int maxDepth(TreeNode* root) {
        if (root == nullptr) return 0;
        int my_max = 0;
        bfs(root, my_max, 0);
        return my_max;
    }
    
    void bfs(TreeNode* root, int& my_max, int depth) {
        if (root == nullptr) return;
        
        if (root->left == nullptr && root->right == nullptr) {
            my_max = max(my_max, depth+1);
        } else {
            bfs(root->left, my_max, depth+1);
            bfs(root->right, my_max, depth+1);    
        }
    }
};