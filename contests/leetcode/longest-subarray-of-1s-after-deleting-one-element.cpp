// https://leetcode.com/problems/longest-subarray-of-1s-after-deleting-one-element/
class Solution {
public:
    int longestSubarray(vector<int>& nums) {
        int current0 = 0;
        int current1 = 0;
        int previous1 = 0;
        int state = 0;
        int best = 0;
        bool zeros = false;
        for (int i = 0; i < nums.size(); ++i) {
            if (nums[i] == 0) {
                zeros = true;
                if (state == 1) {
                    current0 = 0;
                    state = 0;
                }
                current0++;
            } else {
                if (state == 0) {
                    previous1 = current1;
                    current1 = 0;
                    state = 1;
                }
                current1++;
                
                best = max(best, current1);
                if (current0 == 1) 
                    best = max(best, previous1 + current1);
            }
        }
        return best - int(!zeros);
    }
};