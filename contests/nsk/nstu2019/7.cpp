#include <iostream>

using namespace std;

typedef int64_t bint;

int main() {
	bint n, x;
	cin >> n >> x;
	cout << n * (x*(x + 1) / 2 - 1);
}