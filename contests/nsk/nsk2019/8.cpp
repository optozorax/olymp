#include <iostream>
#include <vector>
#include <map>
#include <cmath>
#include <functional>
#include <numeric>
#include <algorithm>

using namespace std;
typedef int64_t bint;

bint count(vector<bint> m, bint l) {
	if (m.size() == 1) return 0;
	sort(m.begin(), m.end(), greater<int>());
	vector<bint> a; bint la = 0;
	vector<bint> b; bint lb = 0;
	for (auto& i : m) {
		if (la < lb) {
			a.push_back(i);
			la += i;
		} else {
			b.push_back(i);
			lb += i;
		}
	}

	return l + count(a, la) + count(b, lb);
}

int main() {
	bint n, l;
	//n = 5;
	//n = 50;
	cin >> l >> n;
	vector<bint> b;
	for (int i = 0; i < n; i++) {
		bint a;
		cin >> a;
		b.push_back(a);
	}
	//l = 4; n = 4;
	//vector<bint> b = { 1, 1, 1, 1 };
	
	cout << count(b, l);

	//system("pause");
}