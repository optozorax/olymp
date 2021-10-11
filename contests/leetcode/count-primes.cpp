// https://leetcode.com/problems/count-primes/
class Solution {
public:
    int countPrimes(int n) {
        vector<bool> er(n+1, false);
        int count = 0;
        for (int i = 2; i < n; ++i) {
            if (er[i] == false) {
                count++;
                int j = i + i;
                while (j < n) {
                    er[j] = true;
                    j += i;
                }
            }
        }
        return count;
    }
};