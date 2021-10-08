// https://leetcode.com/problems/two-sum/
class Solution {
public:
    int binary_search(int a, int b, function<bool(int)> good) {
        int old_b = b;
        if (b-a == 0) return -1;
        while (b-a != 1) {
            int c = (a+b)/2;
            if (good(c)) b = c; else a = c;
        }
        if (good(a)) return a;
        else if (b < old_b && good(b)) return b;
        else return -1;
    }

    vector<int> twoSum(vector<int>& nums, int target) {
        vector<pair<int, int>> n(nums.size());
        for (int i = 0; i < nums.size(); ++i) {
            n[i] = {nums[i], i};
        }
        sort(n.begin(), n.end(), [](const auto& l, const auto& r){ return l.first < r.first; });
        for (int i = 0; i < n.size(); ++i) {
            int pos = binary_search(0, n.size(), [&](int c) { return n[c].first >= target - n[i].first; });
            if (pos != -1 && pos != i) {
                if (n[i].first + n[pos].first == target) 
                    return {n[i].second, n[pos].second};
            }
        }
        return {-1, -1}; // unreachable
    }
};