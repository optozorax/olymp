// https://leetcode.com/problems/longest-substring-without-repeating-characters/
class Solution {
public:
    int lengthOfLongestSubstring(string s) {
        vector<int> count(255, 0);
        
        int best = 0;
        int more = 0;
        int l = 0;
        for (int r = 0; r < s.size(); ++r) {
            count[s[r]]++;
            if (count[s[r]] > 1) more++;
            
            while (more != 0) {
                if (count[s[l]] > 1) more--;
                count[s[l]]--;
                l++;
            }
            
            best = max(best, r-l+1);
        }
        return best;
    }
};