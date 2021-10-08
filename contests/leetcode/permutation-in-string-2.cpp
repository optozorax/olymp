// https://leetcode.com/problems/permutation-in-string/
class Solution {
public:
    bool checkInclusion(string to_find, string where) {
        #define ALPHABET_SIZE 28

        string s = to_find + string(1, char('a' + ALPHABET_SIZE - 1)) + where;
        array<int, ALPHABET_SIZE> count = {};
        for (int i = 0; i < to_find.size(); ++i) {
            count[to_find[i] - 'a']++;
        }

        // modified z-function 
        // https://cp-algorithms.com/string/z-function.html
        int n = (int) s.length();
        vector<int> z(n, 0);
        array<int, ALPHABET_SIZE> z_count = {};
        for (int i = 1, l = 0, r = 0; i < n; ++i) {
            if (i <= r)
                z[i] = min (r - i + 1, z[i - l]);
            while (i + z[i] < n && z_count[s[i + z[i]] - 'a'] < count[s[i + z[i]] - 'a']) {
                z_count[s[i + z[i]] - 'a']++;
                ++z[i];
            }
            if (i + z[i] - 1 > r)
                l = i, r = i + z[i] - 1;

            if (z[i] != 0) z_count[s[i] - 'a']--;
        }

        for (int i = to_find.size(); i < s.size(); ++i) {
            if (z[i] == to_find.size()) return true;
        }
        return false;
    }
};

/* 

решено с использованием z-function

 */