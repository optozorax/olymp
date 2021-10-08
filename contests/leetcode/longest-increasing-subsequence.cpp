// https://leetcode.com/problems/longest-increasing-subsequence/
class Solution {
public:
    // this is magic from https://leetcode.com/problems/longest-increasing-subsequence/discuss/1326552/Optimization-From-Brute-Force-to-Dynamic-Programming-Explained!
    int lengthOfLIS(vector<int>& nums) {
        int len = 0;
        for (auto& i : nums) {
            int result = binary_search(0, len, [&](int c){ return nums[c] >= i; });
            if (result == -1) {
                nums[len] = i;
                len++;
            } else {
                nums[result] = i;
            }
        }
        return len;
    }

    template<typename F> // function<bool(int)>
    int binary_search(int a, int b, F good) {
        int old_b = b;
        if (b-a == 0) return -1;
        while (b-a != 1) {
            int c = (a+b)/2;
            if (good(c)) b = c; else a = c;
        }
        if (good(a)) return a;
        else if (b < old_b && good(b)) return b;
        else return -1;
    }
};

/* 

можно сделать другое решение с помощью ДП, но оно O(n^2), а это мне вообще не нравится. ибо до него додуматься оч сложно.

 */