// https://leetcode.com/problems/max-consecutive-ones-iii/
class Solution {
public:
    int longestOnes(vector<int>& nums, int k) {
        int best = 0;
        int zeros = 0;
        int j = 0;
        for (int i = 0; i < nums.size(); ++i) {
            if (nums[i] == 0) zeros++;
            while (zeros > k) {
                if (nums[j] == 0) zeros--;
                j++;
            }
            
            best = max(best, i - j + 1);
        }
        return best;
    }
};