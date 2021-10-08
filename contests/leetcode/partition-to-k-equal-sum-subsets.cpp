// https://leetcode.com/problems/partition-to-k-equal-sum-subsets/
class Solution {
public:
    bool canPartitionKSubsets(vector<int>& nums, int k) {
        int sum = accumulate(nums.begin(), nums.end(), 0);
        if (sum % k != 0) return false;
        sum /= k;

        vector<int> dp;
        dp.reserve(1 << k);
        dp.push_back(0);
        for (int i = 0; i < nums.size(); ++i) {
            int current_size = dp.size();
            for (int j = 0; j < current_size; ++j) {
                dp.push_back(dp[j] + nums[i]);
            }
        }

        int pow = k >= 8 ? 4 : (k >= 4 ? 3 : (k >= 2 ? 2 : 1));
        vector<unordered_set<int>> pows(pow);
        for (int i = 0; i < dp.size(); ++i) {
            if (dp[i] == sum) pows[0].insert(i);
        }

        auto merge = [](
            const unordered_set<int>& a, 
            const unordered_set<int>& b, 
            unordered_set<int>& into
        ) {
            for (const auto& ae : a) {
                for (const auto& be : b) {
                    if ((ae & be) == 0) {
                        into.insert(ae | be);
                    }
                }
            }
        };
        
        for (int i = 1; i < pow; ++i) {
            merge(pows[i-1], pows[i-1], pows[i]);
        }

        unordered_set<int> result;
        unordered_set<int> new_result;
        result.insert(0);
        int count = 0;
        while (k != 0) {
            if (k % 2 == 1) {
                new_result.clear();
                merge(result, pows[count], new_result);
                swap(new_result, result);
            }
            k /= 2;
            count++;
        }

        return result.size() != 0;
    }
};

/* 

2 subsests has all elements: x1 ^ x2 = 2^k-1
2 subsets has common element: x1 & x2 != 0
2 subsets union: x1 | x2

алгоритм:
    * вычисляем сумму всего массива, определяем делится ли она на k, если нет, выходим, иначе делим её на k
    * динамическим программированием вычисляем сумму на каждом подмножестве и храним это в массиве по адресу битового представления этого множества: vector<int>
    * записываем в отдельный массив vector<int> адреса тех элементов, сумма которых равна sum/k
    * определяем какой порядок нам нужен: 1, 2, 4, 8 (максимальная позиция 1 в битовом представлении)
    * вычисляем всё до нужного порядка и записываем в vector<vector<int>>
    * проходимся по числу k, берём каждый бит и если он равен 1, то обрабатываем в соответствии с нужным числом
    * под конец если размер массива != 0 то это true, иначе false

 */