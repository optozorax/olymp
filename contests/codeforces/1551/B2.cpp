#include <bits/stdc++.h>
using namespace std;
 
template <class T> char *debug_arr(T begin, T end) {
	for (; begin != end; ++begin) {
		cerr << *begin << " ";
	}
	return "";
}
#define DS cerr << "? ";
#define D(A) cerr << #A << ": " << A << ", ";
#define D2(A) cerr << #A << ": " << debug_arr(A.begin(), A.end()) << ", ";
#define DL cerr << endl;
 
int main() {
	int64_t t;
	cin >> t;
	for (int64_t t1 = 0; t1 < t; ++t1) {
		int64_t n, k;
		cin >> n >> k;
 
		map<int64_t, vector<int64_t>> pos;
		vector<int64_t> a(n);
		for (int64_t i = 0; i < n; ++i) {
			cin >> a[i];
			pos[a[i]].push_back(i);
		}
 
		vector<pair<int64_t, int64_t>> s_full;
		for (auto i = pos.begin(); i != pos.end(); ++i) {
			s_full.push_back({(*i).second.size(), (*i).first});
		}
 
		sort(s_full.begin(), s_full.end(), [](auto &left, auto &right) {
			return left.first < right.first;
		});
 
		vector<int64_t> s;
		vector<int64_t> s_rev;
		s.reserve(pos.size());
		s_rev.reserve(pos.size());
		for (int64_t i = 0; i < s_full.size(); ++i) {
			s.push_back(s_full[i].first);
			s_rev.push_back(s_full[i].second);
		}
 
		int64_t s1 = 0, n1 = 0;
		for (int64_t i = 0; i < s.size(); ++i) {
			if (s[i] < k) {
				s1 += s[i];
				n1++;
			} else {
				break;
			}
		}
 
		// DS D(s1) D(n1) DL
 
		int64_t s2 = 0;
		int64_t m;
		for (m = s.size(); m > 0; --m) {
			int64_t first = s.size() - m;
 
			if (s2 + s1 >= n1 * k) break;
			if (s[first] >= k) break;
 
			s2 += s[first];
			s1 -= s[first];
			n1--;
		}

		// DS D(m) DL

		vector<int> c(n, 0);
		int color = 0;
		for (auto i = s_rev.rbegin(); i != s_rev.rend(); ++i) {
			auto vec = pos[*i];
			int count1 = 0;
			for (const auto& j : vec) {
				c[j] = color % k + 1;				
				color++;
				count1++;
				if (color == m * k) {
					goto outer_break;
				}
				if (count1 == k) {
					break;
				}
			}
			// DS D(*i) D2(pos[*i]) D(count1) D(color) D(m * k) DL
		}
		outer_break:

		for (int64_t i = 0; i < c.size(); ++i) {
			cout << c[i] << " ";
		}
		cout << endl;
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
создаём массив цветов, инициализируем его 0
цикл k раз
	i = m
	проходимся сверху-вниз, убираем по одному элементу до тех пор пока i != 0, записываем это в массив цвета
выводим
 
 */
