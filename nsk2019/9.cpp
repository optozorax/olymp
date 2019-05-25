#include <iostream>
#include <vector>
#include <map>
#include <cmath>

using namespace std;
typedef int64_t bint;

map<bint, bint> getDelitels(bint a) {
	map<bint, bint> result;
	bint max = sqrt(a);
	for (bint i = 2; i <= max + 1; i++) {
		if (a % i == 0) {
			result[i]++;
			a /= i;
			i--;
		}
	}
	return result;
}

bool getAnswer(bint n, bint k) {
	auto res = getDelitels(n);
	for (auto& i : res) {
		if (i.second >= k)
			return true;
	}
	return false;
}

int  main() {
	bint n, k;
	//n = 4; k = 3;
	//n = 4; k = 2;
	//n = 16, k = 2;
	cin >> n >> k;
	if (getAnswer(n, k))
		cout << "NO";
	else
		cout << "YES";

	//system("pause");
}