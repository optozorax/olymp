// https://leetcode.com/problems/top-k-frequent-elements/
class Solution {
public:
    vector<int> topKFrequent(vector<int>& nums, int k) {
        unordered_map<int, int> count;
        for (auto& i : nums) count[i]++;
        
        multimap<int, int> m;
        for (auto& i : count) {
            m.insert({i.second, i.first});
            if (m.size() > k) {
                m.erase(m.begin());
            }
        }
        
        vector<int> result;
        for (auto& i : m) {
            result.push_back(i.second);
        }
        return result;
    }
};