// https://leetcode.com/problems/letter-combinations-of-a-phone-number/
class Solution {
public:
    const array<string, 8> phone = {"abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"};

    vector<string> letterCombinations(string digits) {
        if (digits == "") return vector<string>();
        vector<string> result;
        string current;
        recur(digits, 0, current, result);
        return result;
    }

    void recur(string& digits, int pos, string& current, vector<string>& result) {
        if (pos == digits.size()) {
            result.push_back(current);
        } else {
            for (const auto& i : phone[digits[pos] - '2']) {
                current.push_back(i);
                recur(digits, pos+1, current, result);
                current.pop_back();
            }
        }
    }
};