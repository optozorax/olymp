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

int main() {
	int t;
	cin >> t;
	for (int t1 = 0; t1 < t; ++t1) {
		int n, m, k;
		cin >> n >> m >> k;

		if (solve(n, m, k)) {
			cout << "YES" << endl;
		} else {
			cout << "NO" << endl;
		}
	}
}
