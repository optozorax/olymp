// https://leetcode.com/problems/unique-paths/
class Solution {
public:
    int uniquePaths(int m, int n) {
        m--;
        n--;
        int64_t answer = 1;
        int i = 1;
        int j = 1;
        for (int k = 1; k <= m + n; ++k) {
            answer *= k;
            if (i <= m && answer % i == 0) {
                answer /= i;
                i++;
            }
            if (j <= n && answer % j == 0) {
                answer /= j;
                j++;
            }
        }
        return answer;
    }
};