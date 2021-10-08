// https://leetcode.com/problems/3sum-closest/
class Solution {
public:
    int threeSumClosest(vector<int>& nums, int target) {
        sort(nums.begin(), nums.end());
        int best = nums[0]+nums[1]+nums[2] - target;
        for (int i = 0; i < nums.size(); ++i) {
            for (int j = i+1; j < nums.size(); ++j) {
                int pos = binary_search(0, nums.size(), [&] (int c) {
                    return nums[i] + nums[j] + nums[c] >= target;
                });
                if (pos == -1) pos = nums.size()-1;
                auto process = [&](int pos){
                    if (pos >= 0 && pos < nums.size()) {
                        if (pos != i && pos != j) {
                            int sum = nums[i] + nums[j] + nums[pos] - target;
                            if (abs(sum) < abs(best)) {
                                best = sum;
                            }
                        }
                    }
                };
                process(pos-1);
                process(pos);
                process(pos+1);
            }
        }
        return best + target;
    }
    
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
};