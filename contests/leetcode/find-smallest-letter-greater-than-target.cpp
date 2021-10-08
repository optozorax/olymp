// https://leetcode.com/problems/find-smallest-letter-greater-than-target/
class Solution {
public:
    char nextGreatestLetter(vector<char>& letters, char target) {        
        int a = 0, b = letters.size();
        while (a < b) {
            int c = (b+a)/2;
            if (letters[c] <= target) {
                a = c+1;
            } else {
                b = c;
            }
        }
        return letters[a % letters.size()];
    }
};