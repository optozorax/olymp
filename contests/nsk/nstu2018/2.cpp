#include <iostream>
using namespace std;
typedef int64_t bint;

bint get(bint a, bint b, bint c) {
	if (a == 0) {
		if (b == 0) {
			if (c == 0) {
				return -1;
			} else {
				return 0;
			}
		} else {
			return 1;
		}
	} else {
		bint d = b*b-4*a*c;
		if (d == 0) {
			return 1;
		} else {
			if (d < 0) {
				return 0;
			} else {
				return -1;
			}
		}
	}
} 

int main() {
	bint a, b, c;
	cin >> a >> b >> c;
	cout << get(a, b, c);

	//system("pause");
}