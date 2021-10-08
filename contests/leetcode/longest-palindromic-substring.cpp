// https://leetcode.com/problems/longest-palindromic-substring/
class Solution {
public:
    string longestPalindrome(string s) {
        auto get_s = [&s](int pos) {
            return pos % 2 == 1 ? '#' : s[pos / 2];
        };
        int n = s.size() * 2 - 1;
        vector<int> z(n, 0);
        int l = 0;
        int r = 0;
        for (int i = 0; i < n; ++i) {
            if (i <= r) z[i] = min(r-i, z[l+r-i]);
            while (
                i + z[i] + 1 > r && 
                i + z[i] + 1 < n && 
                i - z[i] - 1 >= 0 && 
                get_s(i + z[i] + 1) == get_s(i - z[i] - 1)
            ) {
                z[i]++;
            }
            if (i + z[i] > r) {
                r = i + z[i];
                l = i - z[i];
            }
        }
        int pos = 0;
        int best = 0;
        for (int i = 0; i < n; ++i) {
            int size = ((z[i] + i % 2) / 2) * 2 + (1 - i % 2);
            if (size > best) {
                pos = i;
                best = size;
            }
        }
        int start = (pos - z[pos] + 1) / 2;
        return s.substr(start, best);
    }
};