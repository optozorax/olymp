#include <iostream>
using namespace std;
typedef int64_t bint;

int main() {
	bint x, k;
	cin >> x >> k;
	x = x << k;
	cout << x;
}