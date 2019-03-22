#include <iostream>

using namespace std;

int main() {
	int n;
	cin >> n;
	int current, last = 0, result = 1;
	for (int i = 0; i < n; ++i) {
		cin >> current;
		result = last + result - current;
		if (last >= current)
			result++;
		if (result <= 0)
			result = 1;
		last = current;
	}

	cout << result;
}