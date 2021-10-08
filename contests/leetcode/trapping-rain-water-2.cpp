// https://leetcode.com/problems/trapping-rain-water/
class Solution {
public:
    int trap(vector<int>& h) {
        int n = h.size();
        int l = 0;
        int r = n - 1;
        int l_max = h[l];
        int r_max = h[r];
        int sum = 0;
        while (l < r) {
            if (h[l] < h[r]) {
                l++;
                if (min(l_max, r_max) > h[l])
                    sum += min(l_max, r_max) - h[l];
                l_max = max(l_max, h[l]);
            } else {
                r--;
                if (min(l_max, r_max) > h[r])
                    sum += min(l_max, r_max) - h[r];
                r_max = max(r_max, h[r]);
            }
        }
        return sum;
    }
};