// https://leetcode.com/problems/peak-index-in-a-mountain-array/
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
    
    int peakIndexInMountainArray(vector<int>& arr) {
        return binary_search(0, arr.size()-1, [&](int c){ return arr[c]-arr[c+1] > 0; });
    }
};