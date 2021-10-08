// https://leetcode.com/problems/find-k-closest-elements/
class Solution {
public:
    vector<int> findClosestElements(vector<int>& arr, int k, int x) {
        int l = 0;
        int r = k-1;
        for (int i = k; i < arr.size(); ++i) {
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
};