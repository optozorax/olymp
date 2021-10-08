// https://leetcode.com/problems/number-of-longest-increasing-subsequence/
class Solution {
public:
    int findNumberOfLIS(vector<int>& nums) {
        vector<vector<pair<int, int>>> d;
        for (auto& i : nums) {
            int pos = binary_search(0, d.size(), [&] (int c) {
                return d[c].back().first >= i;
            });
            
            if (pos == -1) pos = d.size();
            
            int count = 0;
            if (pos == 0) {
                count = 1;
            } else {
                for (auto& j : d[pos-1]) {
                    if (j.first < i) {
                        count += j.second;
                    }
                }
            }
            
            if (pos == d.size()) {
                d.push_back({make_pair(i, count)});
            } else {
                d[pos].push_back(make_pair(i, count));
            }
        }
        int answer = 0;
        for (auto& i : d.back()) answer += i.second;
        return answer;
    }
    
    template<typename F>
    int binary_search(int a, int b, F good) {
        int old_b = b;
        if (b-a == 0) return -1;
        if (good(a)) return a;
        if (!good(b-1)) return -1;
        while (b-a != 1) {
            int c = (a+b)/2;
            if (good(c)) b = c; else a = c;
        }
        if (good(a)) return a;
        else if (b < old_b && good(b)) return b;
        else return -1;
    }
};