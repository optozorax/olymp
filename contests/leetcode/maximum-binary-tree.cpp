// https://leetcode.com/problems/maximum-binary-tree/
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
    TreeNode* constructMaximumBinaryTree(const vector<int>& nums) {
        if (nums.size() == 0) return nullptr;
        
        int max_pos = distance(nums.begin(), max_element(nums.begin(), nums.end()));
        
        return new TreeNode(
            nums[max_pos],
            constructMaximumBinaryTree(vector<int>(nums.begin(), nums.begin() + max_pos)),
            constructMaximumBinaryTree(vector<int>(nums.begin() + max_pos + 1, nums.end()))
        );
    }
};