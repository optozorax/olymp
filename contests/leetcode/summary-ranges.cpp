// https://leetcode.com/problems/summary-ranges/
class Solution {
public:
    vector<string> summaryRanges(vector<int>& nums) {
        vector<string> result;
        int i = 0;
        while (i < nums.size()) {
            int start = nums[i];
            int64_t current = nums[i];
            i++;
            current++;
            while (i < nums.size() && nums[i] == current) {
                i++;
                current++;
            }
            int end = current - 1;
            
            if (start == end) {
                result.push_back(to_string(start));
            } else {
                result.push_back(to_string(start) + "->" + to_string(end));
            }
        }
        return result;
    }
};