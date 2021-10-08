// https://leetcode.com/problems/majority-element/
class Solution {
public:
    int majorityElement(vector<int>& nums) {
        int count = 1;
        int current = nums[0];
        for (int i = 1; i < nums.size(); ++i) {
            if (nums[i] == current) {
                count++;
            } else {
                count--;
                if (count == 0) {
                    current = nums[i];
                    count = 1;
                }
            }
        }
        return current;
    }
};

/* 

Boyer-Moore Voting Algorithm

*/