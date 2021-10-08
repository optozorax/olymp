// https://leetcode.com/problems/minimum-window-substring/
class Solution {
public:
    string minWindow(string s, string t) {
        vector<int> ct(255, 0);
        for (auto& i : t) ct[i]++;
        
        int count = 0;
        for (auto& i : ct) {
            if (i > 0) 
                count++;
        }
        
        bool first = true;
        int lbest = 0;
        int rbest = 0;
        int same = 0;
        int r = 0;
        vector<int> cs(255, 0);
        for (int l = 0; l < s.size(); ++l) {
            while (r != s.size() && same != count) {
                cs[s[r]]++;
                if (ct[s[r]] != 0 && cs[s[r]] == ct[s[r]]) same++;
                r++;
            }
            
            if (same == count) {
                if (first || r - l < rbest - lbest) {
                    lbest = l;
                    rbest = r;
                    first = false;
                }
            }
            
            if (ct[s[l]] != 0 && cs[s[l]] == ct[s[l]]) same--;
            cs[s[l]]--;
        }
        
        if (first) {
            return string();
        } else {
            return s.substr(lbest, rbest-lbest);
        }
    }
};