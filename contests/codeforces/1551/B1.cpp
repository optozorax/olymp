#include <bits/stdc++.h>
using namespace std;

int main() {
	int t;
	cin >> t;
	for (int t1 = 0; t1 < t; ++t1) {
		string str;
		cin >> str;
		int k = 2;
		int n = str.size();

		map<int, vector<int>> pos;
		vector<int> a(n);
		for (int i = 0; i < n; ++i) {
			a[i] = str[i];
			pos[a[i]].push_back(i);
		}

		vector<int> s;
		for (auto i = pos.begin(); i != pos.end(); ++i) {
			s.push_back((*i).second.size());
		}

		sort(s.begin(), s.end());

		int s1 = 0, n1 = 0;
		for (int i = 0; i < s.size(); ++i) {
			if (s[i] < k) {
				s1 += s[i];
				n1++;
			} else {
				break;
			}
		}

		int s2 = 0;
		int m;
		for (m = s.size(); m > 0; --m) {
			int first = s.size() - m;

			if (s2 + s1 >= n1 * k) break;
			if (s[first] >= k) break;

			s2 += s[first];
			s1 -= s[first];
			n1--;
		}

		cout << m << endl;
	}
}

/* 

считаем количество встречаемости каждого числа в мапу pos с позицией этого числа
записываем это в отсортированный массив s
посчитать сумму и количество всех чисел < k в сумму s1, n1
находим m такое чтобы суммой минимальных элементов можно было добить всё остальное до числа k
	цикл по m от |s| до 1
		если текущее минимальное число >= k, то найдено
		если нет, то добавляем его в сумму s2, убираем из суммы s1, убираем из n1
		если s2 + s1 >= n1 * k то найдено

 */