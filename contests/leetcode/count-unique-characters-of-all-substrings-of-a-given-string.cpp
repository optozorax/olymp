// https://leetcode.com/problems/count-unique-characters-of-all-substrings-of-a-given-string/
class Solution {
public:
    int uniqueLetterString(string s) {
        vector<int> m(26, 0);
        vector<int> prev(26, 0);
        
        int count = 0;
        int sum = 0;
        for (int i = 0; i < s.size(); ++i) {
            int v = s[i] - 'A';
            if (m[v] < i+1) {
                sum -= m[v];
                m[v] = i+1 - prev[v];
                sum += i+1 - prev[v];
            }
            prev[v] = i + 1;
            count += sum;
        }
        return count;
    }
};