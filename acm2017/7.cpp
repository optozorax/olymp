#include <iostream>

using namespace std;

typedef int64_t bint;

bint nod(bint a, bint b) {
	if (b == 0)
		return a;
	else
		return nod(b, a % b);
}

int main() {
	bint n, m, a, b, o;
	cin >> n >> m;
	a = n*m;
	b = m - n;
	o = nod(a, b);
	a /= o;
	b /= o;
	cout << a << "/" << b;
	return 0;
}