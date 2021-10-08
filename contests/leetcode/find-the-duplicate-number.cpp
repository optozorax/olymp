// https://leetcode.com/problems/find-the-duplicate-number/
class Solution {
public:
    int findDuplicate(vector<int>& nums) {
    	int n = nums.size() - 1;

    	int a = 0;
    	int b = n+1;
    	while (b-a != 1) {
    		int c = (a+b)/2;
    		if (count_less_than(nums, c) - c > 0) {
    			b = c;
    		} else {
    			a = c;
    		}
    	}
    	return b;
    }

    int count_less_than(vector<int>& nums, int greater) {
    	int count = 0;
    	for (int i = 0; i < nums.size(); ++i) {
    		if (nums[i] <= greater) count++;
    	}
    	return count;
    } 
};