// https://leetcode.com/problems/minimum-depth-of-binary-tree/
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
    int minDepth(TreeNode* root) {
        if (root == nullptr) return 0;
        int my_min = 1000000;
        bfs(root, my_min, 0);
        return my_min;
    }
    
    void bfs(TreeNode* root, int& my_min, int depth) {
        if (root == nullptr) return;
        
        if (root->left == nullptr && root->right == nullptr) {
            my_min = min(my_min, depth+1);
        } else {
            bfs(root->left, my_min, depth+1);
            bfs(root->right, my_min, depth+1);    
        }
    }
};