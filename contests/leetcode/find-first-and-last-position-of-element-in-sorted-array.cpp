// https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array/
class Solution {
public:
    vector<int> searchRange(vector<int>& nums, int target) {
        auto first = lower_bound(nums.begin(), nums.end(), target);
        auto last = upper_bound(nums.begin(), nums.end(), target);
        if (first == last || first == nums.end()) return {-1, -1};
        return {int(distance(nums.begin(), first)), int(distance(nums.begin(), last)) - 1};
    }
};