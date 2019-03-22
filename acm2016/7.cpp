#include <iostream>
#include <map>
#include <vector>
#include <cmath>
#include <algorithm>
#include <cstdint>

using namespace std;

vector<pair<long, long>> factor(long a) {
	// Раскладывает на множители
	vector<pair<long, long>> factors;
	factors.push_back({1, 1});
	int start = 2;
	again:
	for (int i = start; i <= sqrt(a); ++i) {
		if (a % i == 0) {
			factors.push_back({i, 0});
			while (a % i == 0) {
				factors.back().second++;
				a /= i;
				if (a == 1)
					break;
			}
			start = i+1;
			goto again;
		}
	}
	if (a != 1)
		factors.push_back({a, 1});
	return factors;
}

long gcd(int64_t a, int64_t b) {
	if (b == 0)
       return a; 
    else
       return gcd(b, a % b);
}

bool is_solve_exists_gauss(vector<vector<int64_t>> A, vector<int64_t> b, int n, int m) {
	// Метод Гаусса
	int count = 0;
	for (int i = 0; i < n; ++i) {
		// Находим строку с минимальным числом
		int64_t min = 0;
		int minj = count;
		for (int j = count; j < m; ++j) {
			if (A[j][i] != 0) {
				if (min == 0) {	
					minj = j;
					min = A[j][i];
				} else {
					if (abs(A[j][i]) < abs(min)) {
						minj = j;
						min = A[j][i];
					}
				}
			}
		}
		if (min != 0) {
			swap(A[minj], A[count]);
			swap(b[minj], b[count]);
		} else
			continue;

		// Отнимаем эту строку от всех остальных
		for (int j = 0; j < m; ++j) {
			if (A[j][i] != 0 && j != count) {
				int64_t m1 = A[count][i]/gcd(A[count][i], A[j][i]);
				int64_t m2 = A[j][i]/gcd(A[count][i], A[j][i]);
				for (int k = 0; k < n; ++k)
					A[j][k] = A[j][k]*m1 - A[count][k]*m2;
				b[j] = b[j]*m1 - b[count]*m2;
			}
		}
		count++;
	}

	// Удаляем нулевые строки
	for (int i = m-1; i >= 0; --i) {
		bool zero_line = true;
		for (int j = 0; j < n; ++j) {
			zero_line &= A[i][j] == 0;
		}
		if (b[i] != 0 && zero_line)
			return false;
		if (!zero_line) {
			m = i+1;
			b.erase(b.begin() + i+1, b.end());
			A.erase(A.begin() + i+1, A.end());
			break;
		}
	}

	if (m < n)
		// Пусть будет так
		return true;
	if (m == n) {
		// Проверка
		std::vector<int64_t> x(n, 0);
		for (int i = n-1; i >= 0; i--) {
			x[i] = b[i];
			for (int j = i+1; j < n; ++j) {
				x[i] -= A[i][j]*x[j];
			}
			if (x[i] % A[i][i] != 0)
				return false;
			x[i] /= A[i][i];
		}
		return true;
	}

	return false;
}

bool solve_exists(const map<long, vector<int>>& slay, int n) {
	int m = slay.size() - 1;
	vector<vector<int64_t>> A(m, vector<int64_t>(n, 0));
	vector<int64_t> b(m);

	// Формируем матрицу по map
	int count = 0;
	for (auto& i : slay) {
		if (i.first != 1) {
			auto& mas = i.second;
			b[count] = mas[0];
			for (int i = 0; i < n; ++i)
				A[count][i] = mas[i+1];
			count++;
		}
	}

	return is_solve_exists_gauss(A, b, n, m);
}

int main() {
	// Ввод данных
	long n, a, b;
	cin >> n;
	vector<pair<long, long>> details(n, {0, 0});
	for (int i = 0; i < n; ++i)
		cin >> details[i].first >> details[i].second;
	cin >> a >> b;

	// Формируем СЛАУ из множителей всех чисел
	map<long, vector<int>> slay;
	int count = 0;
	for (auto& i : details) {
		auto res = factor(i.first);
		auto res2 = factor(i.second);
		for (auto& j : res) {
			if (slay[j.first].size() == 0)
				slay[j.first].resize(n+1, 0);
			slay[j.first][count+1] += j.second;
		}
		for (auto& j : res2) {
			if (slay[j.first].size() == 0)
				slay[j.first].resize(n+1, 0);
			slay[j.first][count+1] -= j.second;
		}
		count++;
	}
	auto res = factor(b);
	auto res2 = factor(a);
	for (auto& j : res) {
		if (slay[j.first].size() == 0)
			slay[j.first].resize(n+1, 0);
		slay[j.first][0] += j.second;
	}
	for (auto& j : res2) {
		if (slay[j.first].size() == 0)
			slay[j.first].resize(n+1, 0);
		slay[j.first][0] -= j.second;
	}

	if (solve_exists(slay, n))
		cout << "YES";
	else
		cout << "NO";
}