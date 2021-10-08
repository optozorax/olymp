// https://leetcode.com/problems/partition-equal-subset-sum/
class Solution {
public:
    bool canPartition(vector<int>& nums) {
        int sum = 0;
        for (auto& i : nums) sum += i;
        
        if (sum % 2 == 1) return false;
        int half = sum / 2;
        int remains = sum;
        
        unordered_set<int> sums;
        sums.insert(0);
        
        unordered_set<int> new_sums;
        for (auto& i : nums) {
            new_sums.clear();
            for (auto& j : sums) {
                if (j + remains >= half) {
                    new_sums.insert(j);
                    new_sums.insert(j + i);
                    if (j + i == half) return true; 
                }
            }
            remains -= i;
            swap(sums, new_sums);
        }
        
        return false;
    }
};