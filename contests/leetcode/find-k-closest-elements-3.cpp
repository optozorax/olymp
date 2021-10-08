// https://leetcode.com/problems/find-k-closest-elements/
class Solution {
public:
    vector<int> findClosestElements(vector<int>& arr, int k, int x) {
        int start = binary_search(0, arr.size(), [&](int c) { return arr[c] >= x; });
        int l = min(int(arr.size() - k), max(0, start-k));
        int r = l + k-1;
        for (int i = r+1; i < arr.size(); ++i) {
            if (r+1 != i) break;
            if (
               (abs(arr[i] - x) < abs(arr[l] - x)) ||
               (abs(arr[i] - x) == abs(arr[l] - x) && arr[i] == arr[l])
            ) {
                l++;
                r++;
            }
        }
        vector<int> result(k);
        for (int i = l; i <= r; ++i) {
            result[i-l] = arr[i];            
        }
        return result;
    }
    
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
};