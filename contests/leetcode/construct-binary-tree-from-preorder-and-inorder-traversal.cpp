// https://leetcode.com/problems/construct-binary-tree-from-preorder-and-inorder-traversal/
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
    TreeNode* buildTree(const vector<int>& preorder, const vector<int>& inorder) {
        /* 
            (b) Preorder (Root, Left, Right)
            (a) Inorder (Left, Root, Right)
        */
        if (preorder.size() == 0) return nullptr;   
        
        auto root = preorder[0];
        auto result = new TreeNode(root);
        
        
        int pos = distance(inorder.begin(), find(inorder.begin(), inorder.end(), root));
        result->left = buildTree(
            vector<int>(preorder.begin() + 1, preorder.begin() + pos + 1),
            vector<int>(inorder.begin(), inorder.begin() + pos)
        );
        result->right = buildTree(
            vector<int>(preorder.begin() + pos + 1, preorder.end()),
            vector<int>(inorder.begin() + pos + 1, inorder.end())
        );
        
        return result;
    }
};