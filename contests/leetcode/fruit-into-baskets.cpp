// https://leetcode.com/problems/fruit-into-baskets/
class Solution {
public:
    int totalFruit(vector<int>& fruits) {
        int current = -1;
        int current_count = 0;
        
        int previous = -1;
        int previous_count = 0;
        
        int best = 0;
        
        for (const auto& i : fruits) {
            if (i == previous) {
                previous = current;
                previous_count += current_count;
                
                current = i;
                current_count = 0;
            } else if (i != current) {
                previous = current;
                previous_count = current_count;
                
                current = i;
                current_count = 0;
            }
            
            if (i == current) {
                current_count++;
                best = max(best, current_count + previous_count);
            }
        }
        return best;
    }
};