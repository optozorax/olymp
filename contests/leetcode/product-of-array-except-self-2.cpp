// https://leetcode.com/problems/product-of-array-except-self/
class Solution {
public:
    vector<int> productExceptSelf(vector<int>& nums) {
        vector<int> answer(nums.size(), 0);
        for (int i = 0; i < nums.size(); ++i) {
            if (i != 0) {
                answer[i] = nums[i] * answer[i-1];
            } else {
                answer[i] = nums[i];
            }
        }

        int suffix = 1;

        for (int i = 0; i < nums.size(); ++i) {
            int i_rev = nums.size() - 1 - i;

            int previous = 1;
            if (i_rev != 0) previous = answer[i_rev-1];

            answer[i_rev] = suffix * previous;

            suffix *= nums[i_rev];
        }

        return answer;
    }
};