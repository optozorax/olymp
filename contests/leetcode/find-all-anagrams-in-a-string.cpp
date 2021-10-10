// https://leetcode.com/problems/find-all-anagrams-in-a-string/
class Solution {
public:
    vector<int> findAnagrams(string s2, string s1) {
        if (s2.size() < s1.size()) return {};

        array<int, 26> count1 = {};
        array<int, 26> count2 = {};

        for (int i = 0; i < s1.size(); ++i) {
            count1[s1[i] - 'a']++;
            count2[s2[i] - 'a']++;
        }

        int same = 0;
        for (int i = 0; i < 26; ++i) {
            if (count1[i] == count2[i]) {
                same++;
            }
        }
        
        vector<int> result;
        if (same == 26) result.push_back(0);

        for (int i = s1.size(); i < s2.size(); ++i) {
            int previous = s2[i-s1.size()] - 'a';
            int previous_diff = count1[previous] - count2[previous];
            if (previous_diff == -1) same++;
            if (previous_diff == 0) same--;
            count2[previous]--;

            int next = s2[i] - 'a';
            int next_diff = count1[next] - count2[next];
            if (next_diff == 0) same--;
            if (next_diff == 1) same++;
            count2[next]++;            
            
            if (same == 26) result.push_back(i+1 - s1.size());
        }

        return result;
    }
};