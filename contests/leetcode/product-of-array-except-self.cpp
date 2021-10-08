// https://leetcode.com/problems/product-of-array-except-self/
class Solution {
public:
    vector<int> productExceptSelf(vector<int>& nums) {
        vector<int> prefix(nums.size(), 0);
        for (int i = 0; i < nums.size(); ++i) {
            if (i != 0) {
                prefix[i] = nums[i] * prefix[i-1];
            } else {
                prefix[i] = nums[i];
            }
        }

        vector<int> suffix(nums.size(), 0);
        for (int i = 0; i < nums.size(); ++i) {
            int i_rev = nums.size() - 1 - i;
            if (i != 0) {
                suffix[i_rev] = nums[i_rev] * suffix[i_rev+1];
            } else {
                suffix[i_rev] = nums[i_rev];
            }
        }

        vector<int> answer(nums.size(), 0);
        for (int i = 0; i < nums.size(); ++i) {
            int previous = 1;
            if (i != 0) {
                previous = prefix[i-1];
            }

            int next = 1;
            if (i + 1 != nums.size()) {
                next = suffix[i+1];
            }

            answer[i] = previous * next;
        }

        return answer;
    }
};