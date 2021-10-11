// https://leetcode.com/problems/intersection-of-two-arrays-ii/
class Solution {
public:
    vector<int> intersect(vector<int>& nums1, vector<int>& nums2) {
        unordered_map<int, int> first;
        for (auto& i : nums1) first[i]++;
        
        vector<int> result;
        for (auto& i : nums2) {
            if (first[i] > 0) {
                result.push_back(i);
                first[i]--;
            }
        }
        return result;
    }
};