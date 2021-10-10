// https://leetcode.com/problems/group-anagrams/
class Solution {
public:
    vector<vector<string>> groupAnagrams(vector<string>& strs) {
        map<array<char, 26>, vector<string>> result;
        for (const auto& i : strs) {
            array<char, 26> count = {};
            for (const auto& j : i) count[j - 'a']++;
            result[count].push_back(i);
        }
        vector<vector<string>> res;
        for (auto& i : result) res.push_back(i.second);
        return res;
    }
};