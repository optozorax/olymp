// https://leetcode.com/problems/sliding-window-median/
class Solution {
public:
    vector<double> medianSlidingWindow(vector<int>& nums, int k) {
        int n = nums.size();
        
        vector<int> first(k, 0);
        for (int i = 0; i < k; ++i) {
            first[i] = nums[i];
        }
        sort(first.begin(), first.end());
        
        multiset<int> left;
        multiset<int> right;
        for (int i = 0; i < k; ++i) {
            if (i < (k+1)/2) {
                left.insert(first[i]);
            } else {
                right.insert(first[i]);
            }
        }
        
        vector<double> result;
        result.reserve(n-k+1);
        
        auto median = [&]() {
            if (k % 2 == 0) {
                return (double(*left.rbegin()) + double(*right.begin())) / 2.;
            } else {
                return double(*left.rbegin());
            }
        };
        
        result.push_back(median());
        
        for (int i = k; i < n; ++i) {
            int to_delete = nums[i-k];
            int to_add = nums[i];
            
            int left_count = 0;
            int right_count = 0;
            
            if (to_delete > *left.rbegin()) {
                right.erase(right.find(to_delete));
                right_count--;
            } else {
                left.erase(left.find(to_delete));
                left_count--;
            }
            
            bool insert_to_right = false;
            if (k == 1) insert_to_right = false;
            if (k > 1 && left.size() != 0 && to_add > *left.rbegin()) insert_to_right = true;
            if (k > 1 && left.size() == 0 && to_add > *right.rbegin()) insert_to_right = true;
            if (insert_to_right) {
                right.insert(to_add);
                right_count++;
            } else {
                left.insert(to_add);
                left_count++;
            }
            
            if (left_count == -1 && right_count == 1) {
                left.insert(*right.begin());
                right.erase(right.find(*right.begin()));
            } else if (left_count == 1 && right_count == -1) {
                right.insert(*left.rbegin());
                left.erase(left.find(*left.rbegin()));
            } else {
                // good
            }
            
            result.push_back(median());
        }
        
        return result;
    }
};