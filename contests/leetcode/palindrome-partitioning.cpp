// https://leetcode.com/problems/palindrome-partitioning/
class Solution {
public:
    vector<vector<string>> partition(string s) {
        int n = s.size();
        vector<vector<bool>> dp(n, vector<bool>(n, false));
        for (int i1 = 0; i1 < n; ++i1) {
            int i = n - 1 - i1;
            for (int j = 0; j < n; ++j) {
                dp[i][j] = s[i] == s[j];
                if (i+1 != n && j > 0 && i+1 < j) {
                    dp[i][j] = dp[i][j] && dp[i+1][j-1];
                }
            }
        }

        vector<string> current;
        vector<vector<string>> result;
        recur(dp, s, 0, current, result);
        return result;
    }

    void recur(
        const vector<vector<bool>>& dp,
        const string& s,
        int pos,
        vector<string>& current,
        vector<vector<string>>& result
    ) {
        if (pos == s.size()) {
            result.push_back(current);
            return;
        }

        for (int i = pos; i < s.size(); ++i) {
            if (dp[pos][i]) {
                current.push_back(s.substr(pos, i - pos + 1));
                recur(dp, s, i+1, current, result);
                current.pop_back();
            }
        }
    }
};

/* 

dp[i][j] = dp[i+1][j-1] && s[i] == s[j];

 */