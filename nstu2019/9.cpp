#include <iostream>
#include <cmath>
#include <vector>

using namespace std;

typedef int64_t bint;

bint count(bint a, bint b, bint c) {
	vector<bint> up, down;

	for (int i = 2; i <= a + b + c; ++i) up.push_back(i);
	for (int i = 2; i <= a; ++i) down.push_back(i);
	for (int i = 2; i <= b; ++i) down.push_back(i);
	for (int i = 2; i <= c; ++i) down.push_back(i);

	while (up.size() > 1) {
		for (auto& i : up) {
			start:
			for (int j = 0; j < down.size(); ++j) {
				if (i % down[j] == 0) {
					i /= down[j];
					down.erase(down.begin() + j);
					goto start;
				}
			}
		}
		up[0] *= up[1];
		up.erase(up.begin() + 1);
	}

	for (auto& i : up) {
		start1:
		for (int j = 0; j < down.size(); ++j) {
			if (i % down[j] == 0) {
				i /= down[j];
				down.erase(down.begin() + j);
				goto start1;
			}
		}
	}

	if (down.size() > 0)
		throw exception();

	return up[0];
}

int main() {
	bint n, m, k;
	cin >> n >> m >> k;
	cout << count(n, m, k);
	//system("pause");
}