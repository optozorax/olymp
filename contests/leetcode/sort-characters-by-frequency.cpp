// https://leetcode.com/problems/sort-characters-by-frequency/
class Solution {
public:
    string frequencySort(string s) {
        vector<int> count(255, 0);
        for (auto& i : s) count[i]++;
        
        sort(s.begin(), s.end(), [&](const auto& l, const auto& r){
            if (count[l] == count[r]) {
                return l > r; 
            } else {
                return count[l] > count[r];
            }
        });
        
        return s;
    }
};