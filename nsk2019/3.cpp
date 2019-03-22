#include <iostream>
#include <string>
#include <vector>
#include <cmath>

using namespace std;

typedef int64_t bint;

bint count(bint a, bint b, bint px, bint py, bint kx, bint ky) {
	bint res = abs(px - kx) + abs(py - ky);;
	if (a == px && a == kx && ((py < b && b < ky) || (ky < b && b < py))) {
		return res + 2;
	} else if (b == py && b == ky && ((px < a && a < kx) || (kx < a && a < px))) {
		return res + 2;
	} else {
		return res;
	}
}

//#define as(a) cout << #a << ": " << ((bool(a)) ? "true" : "false") << endl;

int main() {
	bint a, b, px, py, kx, ky;
	cin >> a >> b >> px >> py >> kx >> ky;
	cout << count(a, b, px, py, kx, ky);

	/*as(count(0, 1, 0, 0, 1, 1) == 2);
	as(count(0, 1, 0, 0, 0, 2) == 4);
	as(count(1, 0, 0, 0, 2, 0) == 4);
	as(count(0, 2, 0, 0, 0, 1) == 1);
	as(count(2, 0, 0, 0, 1, 0) == 1);

	system("pause");*/
}