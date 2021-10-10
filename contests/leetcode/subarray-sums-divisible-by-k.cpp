// https://leetcode.com/problems/subarray-sums-divisible-by-k/
class Solution {
public:
    int subarraysDivByK(const vector<int>& nums, int k) {
        unordered_map<int, int> m;
        int sum = 0;
        int count = 0;
        m[0] = 1;
        for (const auto& i : nums) {
            sum += i;
            int group = (sum < 0 ? k : 0) + sum % k;
            if (group == k) group = 0;
            count += m[group];
            m[group]++;
        }
        return count;
    }
};