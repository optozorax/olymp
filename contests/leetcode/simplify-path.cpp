// https://leetcode.com/problems/simplify-path/
class Solution {
public:
    string simplifyPath(string path) {
        path += "/";
        vector<string> s;
        string current;
        for (auto& i : path) {
            if (i == '/') {
                if (current == "..") {
                    if (!s.empty()) s.pop_back();
                } else if (current != "" && current != ".") {
                    s.push_back(current);
                }
                current.clear();
            } else {
                current.push_back(i);
            }
        }
        string result = "/";
        for (auto& i : s) {
            result += i;
            result += "/";
        }
        if (result != "/") result.pop_back();
        return result;
    }
};