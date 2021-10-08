// https://leetcode.com/problems/maximize-distance-to-closest-person/
class Solution {
public:
    int maxDistToClosest(vector<int>& seats) {
        int state = 0;
        int count = 0;
        int best = 0;
        
        int first_zeros = 0;
        while (seats[first_zeros] != 1) first_zeros++;
        
        int last_zeros = 0;
        while (seats[seats.size() - 1 - last_zeros] != 1) last_zeros++;
        
        for (int i = first_zeros; i < seats.size() - last_zeros; ++i) {
            if (seats[i] == 0) {
                if (state == 1) {
                    state = 0;
                    count = 0;
                }
                count++;
                best = max(best, count);
            } else {
                if (state == 0) {
                    state = 1;
                }
            }
        }
        
        return max((best + 1) / 2, max(first_zeros, last_zeros));
    }
};