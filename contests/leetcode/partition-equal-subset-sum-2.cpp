// https://leetcode.com/problems/partition-equal-subset-sum/
class Solution {
public:
    bool canPartition(vector<int>& nums) {
        bitset<10001> bits(1);
        int sum = accumulate(nums.begin(), nums.end(), 0);
        for (auto n : nums) bits |= bits << n;
        return !(sum & 1) && bits[sum >> 1];
    }
};

/* 

Решение взято из:
https://leetcode.com/problems/partition-equal-subset-sum/discuss/90590/

 */