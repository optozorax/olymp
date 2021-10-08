// https://leetcode.com/problems/generate-parentheses/
class Solution {
public:
    vector<string> generateParenthesis(int n) {
        vector<string> result;
        string current;
        recur(n, n, 0, current, result);
        return result;
    }
    
    void recur(
        int n, 
        int to_open, 
        int opened, 
        string current, 
        vector<string>& result
    ) {
        if (current.size() == 2*n) {
            result.push_back(current);
            return;
        }
        
        if (to_open > 0) {
            current.push_back('(');
            recur(n, to_open-1, opened+1, current, result);
            current.pop_back();
        }
        
        if (opened > 0) {
            current.push_back(')');
            recur(n, to_open, opened-1, current, result);
            current.pop_back();
        }
    }
};