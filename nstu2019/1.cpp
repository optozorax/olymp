#include <iostream>
#include <string>
#include <bitset>
#include <cctype>

using namespace std;

typedef int64_t bint;

bint let2int(char c) {
	return tolower(c) - 'a';
}

int main() {
	bint m, n, k;
	string as, bs, letter;
	cin >> m >> n >> k;
	cin >> as >> bs >> letter;

	bitset<26> a, b;
	for (auto& i : as) a[let2int(i)] = true;
	for (auto& i : bs) b[let2int(i)] = true;

	bool isFirst = true, isSecond = true;

	for (auto& i : letter) {
		if (isalpha(i)) {
			isFirst &= a[let2int(i)];
			isSecond &= b[let2int(i)];
		}
	}

	if (isFirst && isSecond) {
		cout << "BOTH";
	} else if (!isFirst && !isSecond) {
		cout << "NONE";
	} else if (isFirst) {
		cout << "A";
	} else if (isSecond) {
		cout << "B";
	}
}