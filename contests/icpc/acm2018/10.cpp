#include <iostream>
#include <algorithm>
#include <vector>
#include <string>
#include <fstream>

using namespace std;

typedef int64_t bint;

int main() {
	bint N, K, tmp;
	string str;
	cin >> N >> K >> str;
	int s = 0;

	for (int i = 0; i < K; ++i) {
		cin >> tmp;
		if (tmp < 0)
			tmp = N + tmp;
		s = (s + tmp) % N;
	}

	for (int i = 0; i < N; ++i)
		cout << str[(i + s) % N];
}
