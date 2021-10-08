// https://leetcode.com/problems/search-in-rotated-sorted-array-ii/
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
    
    int search(vector<int>& nums, int target) {
        int first_ends = 1;
        while (first_ends < nums.size() && nums[first_ends] == nums[0]) 
            first_ends++;
        
        if (first_ends == nums.size()) return nums[0] == target;
        
        int start = binary_search(first_ends, nums.size(), [&](int c){ return nums[c] <= nums[0]; });
        
        if (start == -1) start = 0;
        
        int answer = binary_search(0, nums.size(), [&](int c){ 
            return nums[(start + c) % nums.size()] >= target; 
        });
        
        if (answer == -1) return false;
        
        answer = (start + answer) % nums.size();
        
        if (nums[answer] == target) return true;
        else return false;
    }
};