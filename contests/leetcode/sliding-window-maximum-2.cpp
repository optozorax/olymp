// https://leetcode.com/problems/sliding-window-maximum/
class Solution {
public:
    vector<int> maxSlidingWindow(vector<int>& nums, int k) {
        int n = nums.size();
        vector<int> prefix;
        prefix.reserve(n);
        int best = 0;
        for (int i = 0; i < n; ++i) {
            if (i % k == 0) {
                best = nums[i];
            } else {
                best = max(best, nums[i]);
            }
            prefix.push_back(best);
        }

        vector<int> suffix(n, 0);
        best = nums.back();
        for (int i1 = 0; i1 < n; ++i1) {
            int i = n - 1 - i1;
            if (i % k == k-1) {
                best = nums[i];
            } else {
                best = max(best, nums[i]);
            }
            suffix[i] = best;
        }

        vector<int> result;
        result.reserve(n-k+1);
        for (int i = k; i <= n; ++i) {
            result.push_back(max(suffix[i-k], prefix[i-1]));
        }
        return result;
    }
};

/* 

thanks https://leetcode.com/problems/sliding-window-maximum/discuss/951894/

 */