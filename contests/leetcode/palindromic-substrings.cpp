// https://leetcode.com/problems/palindromic-substrings/
class Solution {
public:
    int countSubstrings(string s) {
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
        int sum = 0;
        for (int i = 0; i < n; ++i) {
            int size = ((z[i] + i % 2) / 2) * 2 + (1 - i % 2);
            sum += (size + 1) / 2;
        }
        return sum;
    }
};