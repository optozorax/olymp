// https://leetcode.com/problems/valid-parentheses/
class Solution {
public:
    bool isValid(string s) {
        stack<char> p;
        for (auto& i : s) {
            if (i == '(') p.push('(');
            if (i == ')') {
                if (!p.empty() && p.top() == '(')
                    p.pop();
                else 
                    return false;
            }
                           
            
            if (i == '{') p.push('{');
            if (i == '}') {
                if (!p.empty() && p.top() == '{')
                    p.pop();
                else 
                    return false;
            }
            
            if (i == '[') p.push('[');
            if (i == ']') {
                if (!p.empty() && p.top() == '[')
                    p.pop();
                else 
                    return false;
            }
        }
        return p.empty();
    }
};