// https://leetcode.com/problems/maximum-product-subarray/
class Solution {
public:
    int maxProduct(vector<int>& nums) {
        int previous_pos = 1;
        int previous_neg = 1;
        int best = nums[0];
        for (auto& i : nums) {
            int mul = max(i, max(i * previous_pos, i * previous_neg));
            best = max(best, mul);
            if (i < 0) {
                previous_neg = min(i, i * previous_pos);
            } else {
                previous_neg = i * previous_neg;
            }
            previous_pos = mul;
        }
        return best;
    }
};