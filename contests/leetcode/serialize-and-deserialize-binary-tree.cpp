// https://leetcode.com/problems/serialize-and-deserialize-binary-tree/
/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     TreeNode *left;
 *     TreeNode *right;
 *     TreeNode(int x) : val(x), left(NULL), right(NULL) {}
 * };
 */
class Codec {
public:

    // Encodes a tree to a single string.
    string serialize(TreeNode* root) {
        if (root == nullptr) return "n";
        
        return 
            to_string(root->val) + 
            "(" + serialize(root->left) + ")(" + 
            serialize(root->right) +  ")";
    }

    // Decodes your encoded data to tree.
    TreeNode* deserialize(string s) {
        if (s == "n") return nullptr;
        
        int n = 0;
        if (s[0] == '-') n++;
        while (s[n] >= '0' && s[n] <= '9') n++;
        int val = 0;
        istringstream ss(s.substr(0, n));
        ss >> val;
        
        int l = n+1;
        int count = 1;
        while (count != 0) {
            if (s[l] == '(') count++;
            if (s[l] == ')') count--;
            l++;
        }
        auto left = s.substr(n+1, l-n-2);
        auto right = s.substr(l+1, s.size() - l-2);
        
        return new TreeNode(val, deserialize(left), deserialize(right));
    }
};

// Your Codec object will be instantiated and called as such:
// Codec ser, deser;
// TreeNode* ans = deser.deserialize(ser.serialize(root));