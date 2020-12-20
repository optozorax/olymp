#include <iostream>

using namespace std;

typedef long long bint;

int main() {
	bint N, M, a, sum = 0;
	cin >> N >> M;
	for (int i = 0; i < N; ++i) {
		cin >> a;
		sum += a;
	}

	if (sum % M == 0)
		cout << "YES";
	else
		cout << "NO";

	return 0;
}