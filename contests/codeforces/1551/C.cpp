#include <bits/stdc++.h>
using namespace std;

int main() {
	int t;
	cin >> t;
	for (int t1 = 0; t1 < t; ++t1) {
		int n;
		cin >> n;

		vector<array<int, 6>> counts(n);
		for (int i = 0; i < n; ++i) {
			string s;
			cin >> s;
			for (int j = 0; j < s.size(); ++j) {
				counts[i][s[j]-'a']++;
				counts[i][5]++;
			}
		}

		int my_max = 0;
		for (int i = 0; i < 5; ++i) { // перебираем все буквы
			sort(counts.begin(), counts.end(), [&i](auto &left, auto &right) {
				int left_current = left[i];
				int left_sum = left[5] - left_current;

				int right_current = right[i];
				int right_sum = right[5] - right_current;

				return left_sum + right_current < right_sum + left_current;
			});

			int sum1 = 0;
			int sum2 = 0;
			int j = 0;
			for (; j < counts.size(); ++j) {
				int current = counts[j][i];
				int sum = counts[j][5] - current;

				if (sum1 + current > sum2 + sum) {
					sum1 += current;
					sum2 += sum;
				} else {
					break;
				}
			}

			my_max = max(my_max, j);
		}

		cout << my_max << endl;
	}
}
