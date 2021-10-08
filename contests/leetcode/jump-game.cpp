// https://leetcode.com/problems/jump-game/
class Solution {
public:
    bool canJump(vector<int>& nums) {
        int best = 1;
        for (auto& i : nums) {
            if (best == 0) return false;
            best = max(best-1, i);
        }
        return true;
    }
};