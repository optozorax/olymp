// https://leetcode.com/problems/binary-search/
class Solution {
public:
    template<typename F> // function<bool(int)>
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
    
    int search(vector<int>& nums, int target) {
        int pos = binary_search(0, nums.size(), [&](int c) { return nums[c] >= target; });
        if (pos == -1) return -1;
        if (nums[pos] == target) {
            return pos;
        } else {
            return -1;
        }
    }
};