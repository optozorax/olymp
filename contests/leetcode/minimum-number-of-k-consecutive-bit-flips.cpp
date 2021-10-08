// https://leetcode.com/problems/minimum-number-of-k-consecutive-bit-flips/
class Solution {
public:
    int minKBitFlips(vector<int>& nums, int k) {
        int count = 0;
        deque<int> c;
        for (int i = 0; i < nums.size(); ++i) {
            while (c.size() != 0 && c.front() < i) c.pop_front();
            if ((nums[i] + c.size()) % 2 != 1) {
                count++;
                if (i + k < nums.size() + 1) {
                    c.push_back(i + k - 1);
                } else {
                    return -1;
                }
            }
        }
        return count;
    }
};

/* 

Тесты:
	* все нули
		* чётное число элементов, нечётный k
	* все единицы
		* чётное число элементов, нечётный k
	* сначала n нулей, затем оставшиеся единицы
	* можно отзеркалить текущий тест, тогда ответ не изменится
	* можно инвертировать текущий тест

Решения:
	* проходиться по массиву и если текущей ячейке не нравится сколько раз её инвертировали, то инвертировать отрезок от текущего и вправо. если это невозможно, то вывести что невозможно

Ошибки:
	* Не учёл сложность алгоритма, поэтому превысило по времени

 */