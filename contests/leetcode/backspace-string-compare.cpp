// https://leetcode.com/problems/backspace-string-compare/
class Solution {
public:
    bool backspaceCompare(string s, string t) {
        int a = s.size() - 1, ignore_a = 0;
        int b = t.size() - 1, ignore_b = 0;
        while (true) {
            while (a != -1 && s[a] == '#') {
                ignore_a++;
                a--;
                while (ignore_a != 0 && a != -1) {
                    if (s[a] == '#') 
                        ignore_a++;
                    else 
                        ignore_a--;
                    a--;
                }
            }
            
            while (b != -1 && t[b] == '#') {
                ignore_b++;
                b--;
                while (ignore_b != 0 && b != -1) {
                    if (t[b] == '#') 
                        ignore_b++;
                    else 
                        ignore_b--;
                    b--;
                }
            }
            
            if (a == -1 && b == -1) return true;
            if (a == -1 || b == -1) return false;
            if (s[a] != t[b]) return false;
            
            a--;
            b--;
        }
    }
};