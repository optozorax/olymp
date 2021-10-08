#include <bits/stdc++.h>

using namespace std;

int check(int x) {
	int msk = 0;
	while (x) {
		msk |= (1 << (x % 10));
		x /= 10;
	}
	return __builtin_popcount(msk);
}

int main() {
	int t;
	cin >> t;
	while (t--) {
		int n, k;
		cin >> n >> k;
		while (check(n) > k) {
			int cur = n / 10, mul = 10;
			while (check(cur) > k) {
				cur /= 10;
				mul *= 10;
			}
			mul /= 10;
			n /= mul;
			n++;
			n *= mul;
			cout << "! " << n << endl;
		}
		cout << n << endl;
	}
	return 0;
}