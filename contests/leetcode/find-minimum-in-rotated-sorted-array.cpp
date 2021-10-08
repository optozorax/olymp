// https://leetcode.com/problems/find-minimum-in-rotated-sorted-array/
class Solution {
public:
    template<typename F> // function<bool(int)>
    int binary_search(int a, int b, F good) {
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
    
    int findMin(vector<int>& nums) {
        int start = binary_search(0, nums.size(), [&](int c){ return nums[c] < nums[0]; });
        if (start == -1) start = 0;
        
        return nums[start];
    }
};