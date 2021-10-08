#include <bits/stdc++.h>
using namespace std;

pair<int, int> solve(int a) {
	int c = a / 3;
	int c1 = c, c2 = c;
	if (a - (c1 + 2*c2) == 1) c1 += 1;
	if (a - (c1 + 2*c2) == 2) c2 += 1;
	return {c1, c2};
}

int main() {
	int t;
	cin >> t;
	for (int t1 = 0; t1 < t; ++t1) {
		int a;
		cin >> a;
		auto b = solve(a);
		cout << b.first << " " << b.second << endl;
	}
}
