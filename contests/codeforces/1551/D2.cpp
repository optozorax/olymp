#include <bits/stdc++.h>
using namespace std;

bool solve(int n, int m, int k) {
	if (n % 2 == 1) {
		if (k >= m / 2) {
			if (n == 1) {
				return true;
			}
			n--;
			k -= m / 2;
		} else {
			return false;
		}
	}

	if (m % 2 == 1) m--;

	return k % 2 == 0 && k <= n * (m / 2);
}

vector<vector<int>> draw(int n, int m, int k) {
	vector<char> color; // bitset of colors
	vector<vector<pair<int, bool>>> matrix(m, vector<pair<int, bool>>(n, make_pair<int, bool>(0, false)));
	vector<vector<int>> matrix2(m, vector<int>(n, int(-1)));
	auto h = [&matrix, &color] (int x, int y) { // draw horizontal domino
		int i = color.size();
		color.push_back(0);
		matrix[x][y] = {i, true};
		matrix[x+1][y] = {i, true};
	};
	auto v = [&matrix, &color] (int x, int y) { // draw vertical domino
		int i = color.size();
		color.push_back(0);
		matrix[x][y] = {i, false};
		matrix[x][y+1] = {i, false};
	};
	auto vv = [&v] (int x, int y) { // draw vertical domino twice right
		v(x, y);
		v(x+1, y);
	};
	if (n % 2 == 1) {
		// закрашиваем последнюю горизонтальную строку
		for (int i = 0; i < m / 2; ++i) {
			h(i * 2, n-1);
		}
		n--;
		k -= m / 2;
	}
	if (m % 2 == 1) {
		// закрашиваем последнюю вертикальную строку
		for (int i = 0; i < n / 2; ++i) {
			v(m-1, i * 2);
		}
		m--;
	}

	{
		// закрашиваем вертикальные строки горизонтальными блоками пока не кончится k
		while (k != 0) {
			if (k >= n) {
				for (int i = 0; i < n; ++i) {
					h(m-2, i);
					k--;
				}
				m -= 2;
			} else {
				for (int i = 0; i < k; ++i) {
					h(m-2, i);
				}
				for (int i = 0; i < (n - k) / 2; ++i) {
					vv(m-2, k + i*2);
				}
				k = 0;
				m -= 2;
				break;
			}
		}
	}
	{
		// закрашиваем вертикальные строки вертикальными блоками пока не кончится m*n/2 - k
		while (n != 0) {
			for (int i = 0; i < m; ++i) {
				v(i, n - 2);
			}
			n -= 2;
		}
	}

	for (int i = 0; i < matrix.size(); ++i) {
		for (int j = 0; j < matrix[i].size(); ++j) {
			// если текущая ячейка не закрашена
			if (matrix2[i][j] == -1) {
				// смотрим текущий разрешённый цвет
				int c = color[matrix[i][j].first];
				int c1 = 0; // разрешённый цвет
				while (c % 2 != 0) {
					c /= 2;
					c1++;
				}

				if (matrix[i][j].second) {
					// горизонтальная ячейка

					// закрашиваем им текущую ячейку
					matrix2[i][j] = c1;
					matrix2[i+1][j] = c1;

					// перебираем всех соседей, записываем из запрет цвета
					if (i+2 != matrix.size()) {
						color[matrix[i+2][j].first] |= 1 << c1;
					}
					if (j+1 != matrix[i].size()) {
						color[matrix[i][j+1].first] |= 1 << c1;
						color[matrix[i+1][j+1].first] |= 1 << c1;
					}
					if (j > 0) {
						color[matrix[i][j-1].first] |= 1 << c1;
						color[matrix[i+1][j-1].first] |= 1 << c1;
					}
					if (i > 0) {
						color[matrix[i-1][j].first] |= 1 << c1;
					}
				} else {
					// вертикальная ячейка

					// закрашиваем им текущую ячейку
					matrix2[i][j] = c1;
					matrix2[i][j+1] = c1;

					// перебираем всех соседей, записываем из запрет цвета
					if (j+2 != matrix[i].size()) {
						color[matrix[i][j+2].first] |= 1 << c1;
					}
					if (i+1 != matrix.size()) {
						color[matrix[i+1][j].first] |= 1 << c1;
						color[matrix[i+1][j+1].first] |= 1 << c1;
					}
					if (i > 0) {
						color[matrix[i-1][j].first] |= 1 << c1;
						color[matrix[i-1][j+1].first] |= 1 << c1;
					}
					if (j > 1) {
						color[matrix[i][j-1].first] |= 1 << c1;
					}
				}
			}
		}
	}

	return matrix2;
}

int main() {
	int t;
	cin >> t;
	for (int t1 = 0; t1 < t; ++t1) {
		int n, m, k;
		cin >> n >> m >> k;

		if (solve(n, m, k)) {
			cout << "YES" << endl;
			auto result = draw(n, m, k);
			for (int j = 0; j < n; ++j) {
				for (int i = 0; i < m; ++i) {
					cout << char(result[i][j] + 'a');
				}
				cout << endl;
			}
		} else {
			cout << "NO" << endl;
		}
	}
}
