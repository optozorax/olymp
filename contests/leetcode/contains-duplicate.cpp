// https://leetcode.com/problems/contains-duplicate/
class Solution {
public:
    bool containsDuplicate(vector<int>& nums) {
        set<int> s;
        for (auto i = nums.begin(); i != nums.end(); ++i) {
            if (!s.insert(*i).second) return true;
        }
        return false;
    }
};