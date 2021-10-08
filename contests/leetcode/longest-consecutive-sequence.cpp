// https://leetcode.com/problems/longest-consecutive-sequence/
class Solution {
public:
    int longestConsecutive(vector<int>& nums) {
        unordered_set<int> n;
        for (const auto& i : nums) n.insert(i);
        
        int best = 0;
        for (const auto& i : nums) {
            if (n.find(i-1) == n.end()) {
                int count = 0;
                while (n.find(i + count) != n.end()) {
                    count++;
                }
                best = max(best, count);
            }
        }
        
        return best;
    }
};