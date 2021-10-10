// https://leetcode.com/problems/string-compression/
class Solution {
public:
    int compress(vector<char>& chars) {
        int i = 0;
        int j = 0;
        while (i < chars.size()) {
            int count = 0;
            char current = chars[i];
            while (i < chars.size() && chars[i] == current) {
                count++;
                i++;
            }
            chars[j] = current;
            j++;
            if (count != 1) {
                int size = int(trunc(log10(count))) + 1;
                int k = size-1;
                while (count != 0) {
                    chars[j + k] = count % 10 + '0';
                    count /= 10;
                    k--;
                }
                j += size;
            }
        }
        chars.erase(chars.begin() + j, chars.end());
        return j;
    }
};