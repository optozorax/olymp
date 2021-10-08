// https://leetcode.com/problems/letter-case-permutation/
class Solution {
public:
    vector<string> letterCasePermutation(string s) {
        vector<string> result;
        recur(s, 0, result);
        return result;
    }
    
    void recur(string& s, int i, vector<string>& result) {
        if (i == s.size()) {
            result.push_back(s);
            return;
        }
        
        if (s[i] >= 'a' && s[i] <= 'z') {
            recur(s, i+1, result);
            s[i] = s[i] - 'a' + 'A';
            recur(s, i+1, result);
        } else if (s[i] >= 'A' && s[i] <= 'Z') {
            recur(s, i+1, result);
            s[i] = s[i] - 'A' + 'a';
            recur(s, i+1, result);
        } else {
            recur(s, i+1, result);
        }
    }
};