// https://leetcode.com/problems/sort-colors/
class Solution {
public:
    void sortColors(vector<int>& nums) {
        array<int, 3> colors = {};
        for (auto& i : nums) colors[i]++;
        int pos = 0;
        for (int i = 0; i < colors.size(); ++i) {
            for (int j = 0; j < colors[i]; ++j) {
                nums[pos] = i;
                pos++;
            }
        }
    }
};