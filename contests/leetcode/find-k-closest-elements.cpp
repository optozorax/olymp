// https://leetcode.com/problems/find-k-closest-elements/
class Solution {
public:
    vector<int> findClosestElements(vector<int>& arr, int k, int x) {
        multiset<int> closest;
        for (auto& i : arr) {
            if (i < x) {
                closest.insert(abs(i-x) * 2);
            } else {
                closest.insert(abs(i-x) * 2 + 1);
            }
            if (closest.size() > k) {
                closest.erase(closest.find(*closest.rbegin()));
            }
        }
        vector<int> result;
        for (auto& i : closest) {
            if (i % 2 == 0) {
                result.push_back(x - i/2);
            } else {
                result.push_back(x + i/2);
            }
        }
        sort(result.begin(), result.end());
        return result;
    }
};