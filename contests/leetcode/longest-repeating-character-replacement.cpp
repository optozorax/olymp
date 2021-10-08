// https://leetcode.com/problems/longest-repeating-character-replacement/
class Solution {
public:
    int characterReplacement(string s, int k) {
        unordered_set<char> letters;
        for (auto& i : s) letters.insert(i);
        
        int best = 0;
        for (auto& i : letters) {
            int l = 0;
            int zeros = 0;
            for (int r = 0; r < s.size(); ++r) {
                if (s[r] != i) zeros++;
                while (zeros > k) {
                    if (s[l] != i) zeros--;
                    l++;
                }
                best = max(best, r-l+1);
            }
        }
        return best;
    }
};