// https://leetcode.com/problems/evaluate-reverse-polish-notation/
class Solution {
public:
    int evalRPN(vector<string>& tokens) {
        stack<int> val;
        for (auto& i : tokens) {
            if (i == "+") {
                int a = val.top(); val.pop();
                int b = val.top(); val.pop();
                val.push(b + a);
            } else if (i == "-") {
                int a = val.top(); val.pop();
                int b = val.top(); val.pop();
                val.push(b - a);
            } else if (i == "*") {
                int a = val.top(); val.pop();
                int b = val.top(); val.pop();
                val.push(b * a);
            } else if (i == "/") {
                int a = val.top(); val.pop();
                int b = val.top(); val.pop();
                val.push(b / a);
            } else {
                int v = 0;
                std::istringstream ( i ) >> v;
                val.push(v);
            }
        }
        return val.top();
    }
};