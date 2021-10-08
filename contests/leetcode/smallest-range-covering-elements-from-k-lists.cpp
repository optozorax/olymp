// https://leetcode.com/problems/smallest-range-covering-elements-from-k-lists/
class Solution {
public:
    vector<int> smallestRange(vector<vector<int>>& nums) {
        int lbest = 1000000;
        int rbest = -1000000;
        
        priority_queue<pair<int, int>> pq;
        for (int i = 0; i < nums.size(); ++i) {
            for (int j = 0; j < nums[i].size(); ++j) {
                pq.push(make_pair(-nums[i][j], i));
                lbest = min(lbest, nums[i][j]);
                rbest = max(rbest, nums[i][j]);
            }
        }
        priority_queue<pair<int, int>> pql = pq;
        
        vector<int> count(nums.size(), 0);
        int same = 0;

        bool first = true;
        pair<int, int> rtop = {};
        while (pq.size() != 0) {
            auto ltop = pq.top(); pq.pop();
            ltop.first = -ltop.first;
            
            while (pql.size() != 0 && same != nums.size()) {
                rtop = pql.top(); pql.pop();
                rtop.first = -rtop.first;
                
                if (count[rtop.second] == 0) same++;
                count[rtop.second]++;
            }
            
            if (same == nums.size()) {
                if ((rtop.first - ltop.first < rbest - lbest) || first) {
                    lbest = ltop.first;
                    rbest = rtop.first;
                    first = false;
                }
            }
            
            if (count[ltop.second] == 1) same--;
            count[ltop.second]--;
        }
        
        return {lbest, rbest};
    }
};